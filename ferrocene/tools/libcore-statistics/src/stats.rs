use crate::loc::LOC;
use crate::render::{render_impl, render_type};
use crate::stability::{parse_stability, Stability};
use crate::visitor::Visitor;
use rustdoc_types::Id;
use std::collections::{HashMap, HashSet};

pub(crate) struct StatsCollector {
    loc: LOC,
    seen: HashSet<Id>,
    name_stack: Vec<String>,
    module_stack: Vec<String>,
    stability_stack: Vec<Stability>,
    inside: Inside,
    pub(crate) functions: Vec<Function>,
    pub(crate) types: Vec<Type>,
    pub(crate) type_counters: HashMap<Id, TypeCounters>,
    pub(crate) traits: HashMap<Id, Trait>,
    pub(crate) trait_counters: HashMap<Id, TraitCounters>,
    pub(crate) items: Vec<Item>,
    pub(crate) macros: Vec<Macro>,
}

impl StatsCollector {
    pub(crate) fn new(loc: LOC) -> Self {
        Self {
            loc,
            seen: HashSet::new(),
            name_stack: Vec::new(),
            module_stack: Vec::new(),
            stability_stack: Vec::new(),
            inside: Inside::None,
            functions: Vec::new(),
            types: Vec::new(),
            type_counters: HashMap::new(),
            traits: HashMap::new(),
            trait_counters: HashMap::new(),
            items: Vec::new(),
            macros: Vec::new(),
        }
    }

    fn override_inside(&mut self, f: impl FnOnce(&mut Self)) {
        let old = std::mem::take(&mut self.inside);
        f(self);
        self.inside = old;
    }

    fn gather_common(&self, item: &rustdoc_types::Item) -> Common {
        Common {
            id: item.id.clone(),
            name: self.name_stack.join("::"),
            module: self.module_stack.join("::"),
            public: match &self.inside {
                Inside::TraitImpl { public, .. } => *public,
                Inside::TraitDefinition { public, .. } => *public,
                _ => item.visibility == rustdoc_types::Visibility::Public,
            },
            stability: self.stability_stack.last().cloned(),
        }
    }

    fn type_counters_for<'a>(
        &'a mut self,
        type_: &rustdoc_types::Type,
    ) -> Option<&'a mut TypeCounters> {
        match type_ {
            rustdoc_types::Type::ResolvedPath(path) => {
                Some(self.type_counters.entry(path.id.clone()).or_default())
            }
            _ => None,
        }
    }
}

impl Visitor for StatsCollector {
    fn visit_item(&mut self, crate_: &rustdoc_types::Crate, item: &rustdoc_types::Item) {
        if self.seen.insert(item.id.clone()) {
            let mut pop_name = false;
            let mut pop_stability = false;

            if let Some(name) = &item.name {
                self.name_stack.push(name.clone());
                pop_name = true;
            }
            if let Some(stability) = parse_stability(&item.attrs) {
                self.stability_stack.push(stability);
                pop_stability = true;
            }

            self.walk_item(crate_, item);

            if pop_stability {
                self.stability_stack.pop();
            }
            if pop_name {
                self.name_stack.pop();
            }
        }
    }

    fn visit_impl(
        &mut self,
        crate_: &rustdoc_types::Crate,
        item: &rustdoc_types::Item,
        impl_: &rustdoc_types::Impl,
    ) {
        self.override_inside(|this| {
            let mut signature = String::new();
            render_impl(&mut signature, crate_, impl_);

            if let Some(trait_) = &impl_.trait_ {
                let trait_ = crate_.index.get(&trait_.id).unwrap();
                this.inside = Inside::TraitImpl {
                    type_: impl_.for_.clone(),
                    trait_id: trait_.id.clone(),
                    signature,
                    public: trait_.visibility == rustdoc_types::Visibility::Public,
                };

                if let Some(counters) = this.type_counters_for(&impl_.for_) {
                    let rustdoc_types::ItemEnum::Trait(trait_inner) = &trait_.inner else {
                        panic!("trait is not a trait???");
                    };
                    if trait_inner.is_auto {
                        counters.auto_impls += 1;
                    } else {
                        counters.trait_impls += 1;
                    }
                }
            } else {
                this.inside = Inside::ItemImpl {
                    type_: impl_.for_.clone(),
                    signature,
                };
            }

            if let Some(counters) = this.type_counters_for(&impl_.for_) {
                if impl_.blanket_impl.is_some() {
                    counters.blanket_impls += 1;
                }
            }

            this.walk_impl(crate_, item, impl_);
        })
    }

    fn visit_trait(
        &mut self,
        crate_: &rustdoc_types::Crate,
        item: &rustdoc_types::Item,
        trait_: &rustdoc_types::Trait,
    ) {
        self.override_inside(|this| {
            this.inside = Inside::TraitDefinition {
                id: item.id.clone(),
                public: item.visibility == rustdoc_types::Visibility::Public,
            };

            this.traits.insert(
                item.id.clone(),
                Trait {
                    common: this.gather_common(item),
                    implementations: trait_.implementations.len(),
                },
            );

            this.walk_trait(crate_, item, trait_);
        })
    }

    fn visit_module(
        &mut self,
        crate_: &rustdoc_types::Crate,
        item: &rustdoc_types::Item,
        module: &rustdoc_types::Module,
    ) {
        self.module_stack.push(item.name.as_ref().unwrap().clone());
        self.walk_module(crate_, item, module);
        self.module_stack.pop();
    }

    fn visit_function(
        &mut self,
        _crate: &rustdoc_types::Crate,
        item: &rustdoc_types::Item,
        function: &rustdoc_types::Function,
    ) {
        match self.inside.clone() {
            Inside::None => {}
            Inside::ItemImpl { type_, .. } => {
                if let Some(counters) = self.type_counters_for(&type_) {
                    counters.methods += 1;
                }
            }
            Inside::TraitImpl { type_, .. } => {
                if let Some(counters) = self.type_counters_for(&type_) {
                    counters.trait_methods += 1;
                }
            }
            Inside::TraitDefinition { id, .. } => {
                let counters = self.trait_counters.entry(id.clone()).or_default();
                if function.has_body {
                    counters.default_methods += 1;
                } else {
                    counters.required_methods += 1;
                }
            }
        }

        let span = item.span.as_ref().unwrap();

        self.functions.push(Function {
            common: self.gather_common(item),
            kind: match self.inside {
                Inside::None => FunctionKind::Function,
                Inside::ItemImpl { .. } => FunctionKind::Method,
                Inside::TraitImpl { .. } => FunctionKind::TraitMethod,
                Inside::TraitDefinition { .. } => FunctionKind::TraitMethodDefinition {
                    has_default: function.has_body,
                },
            },
            trait_id: match &self.inside {
                Inside::TraitImpl { trait_id, .. } => Some(trait_id.clone()),
                _ => None,
            },
            impl_: match &self.inside {
                Inside::ItemImpl { signature, .. } => Some(signature.clone()),
                Inside::TraitImpl { signature, .. } => Some(signature.clone()),
                _ => None,
            },
            lines_of_code: self
                .loc
                .stats_for(&span.filename, span.begin.0, span.end.0)
                .unwrap(),
            lines_of_docs: item
                .docs
                .as_ref()
                .map(|d| d.trim().chars().filter(|c| *c == '\n').count())
                .unwrap_or(0),
            file: span.filename.to_str().unwrap().to_string(),
        });
    }

    fn visit_enum(
        &mut self,
        crate_: &rustdoc_types::Crate,
        item: &rustdoc_types::Item,
        enum_: &rustdoc_types::Enum,
    ) {
        self.types.push(Type {
            common: self.gather_common(item),
            kind: TypeKind::Enum,
        });
        self.walk_enum(crate_, item, enum_);
    }

    fn visit_struct(
        &mut self,
        crate_: &rustdoc_types::Crate,
        item: &rustdoc_types::Item,
        struct_: &rustdoc_types::Struct,
    ) {
        self.types.push(Type {
            common: self.gather_common(item),
            kind: TypeKind::Struct,
        });
        self.walk_struct(crate_, item, struct_);
    }

    fn visit_union(
        &mut self,
        crate_: &rustdoc_types::Crate,
        item: &rustdoc_types::Item,
        union_: &rustdoc_types::Union,
    ) {
        self.types.push(Type {
            common: self.gather_common(item),
            kind: TypeKind::Union,
        });
        self.walk_union(crate_, item, union_);
    }

    fn visit_constant(
        &mut self,
        crate_: &rustdoc_types::Crate,
        item: &rustdoc_types::Item,
        constant: &rustdoc_types::Constant,
    ) {
        let mut type_ = String::new();
        render_type(&mut type_, crate_, &constant.type_);

        self.items.push(Item {
            common: self.gather_common(item),
            kind: ItemKind::Const,
            type_,
            value: constant.expr.clone(),
        });
    }

    fn visit_static(
        &mut self,
        crate_: &rustdoc_types::Crate,
        item: &rustdoc_types::Item,
        static_: &rustdoc_types::Static,
    ) {
        let mut type_ = String::new();
        render_type(&mut type_, crate_, &static_.type_);

        self.items.push(Item {
            common: self.gather_common(item),
            kind: ItemKind::Static,
            type_,
            value: static_.expr.clone(),
        });
    }

    fn visit_macro(
        &mut self,
        _crate_: &rustdoc_types::Crate,
        item: &rustdoc_types::Item,
        _macro_: &str,
    ) {
        self.macros.push(Macro {
            common: self.gather_common(item),
            kind: MacroKind::ByExample,
        });
    }

    fn visit_proc_macro(
        &mut self,
        _crate_: &rustdoc_types::Crate,
        item: &rustdoc_types::Item,
        proc_macro: &rustdoc_types::ProcMacro,
    ) {
        self.macros.push(Macro {
            common: self.gather_common(item),
            kind: match &proc_macro.kind {
                rustdoc_types::MacroKind::Bang => MacroKind::ProcBang,
                rustdoc_types::MacroKind::Attr => MacroKind::ProcAttribute,
                rustdoc_types::MacroKind::Derive => MacroKind::ProcDerive,
            },
        })
    }
}

#[derive(Default, Clone)]
enum Inside {
    #[default]
    None,
    ItemImpl {
        type_: rustdoc_types::Type,
        signature: String,
    },
    TraitImpl {
        type_: rustdoc_types::Type,
        trait_id: Id,
        signature: String,
        public: bool,
    },
    TraitDefinition {
        id: Id,
        public: bool,
    },
}

pub(crate) enum FunctionKind {
    Function,
    Method,
    TraitMethod,
    TraitMethodDefinition { has_default: bool },
}

impl std::fmt::Display for FunctionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FunctionKind::Function => f.write_str("function"),
            FunctionKind::Method => f.write_str("method"),
            FunctionKind::TraitMethod => f.write_str("trait method"),
            FunctionKind::TraitMethodDefinition { has_default } => {
                if *has_default {
                    f.write_str("definition of default trait method")
                } else {
                    f.write_str("definition of required trait method")
                }
            }
        }
    }
}

pub(crate) enum TypeKind {
    Struct,
    Enum,
    Union,
}

impl std::fmt::Display for TypeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeKind::Struct => f.write_str("struct"),
            TypeKind::Enum => f.write_str("enum"),
            TypeKind::Union => f.write_str("union"),
        }
    }
}

pub(crate) enum ItemKind {
    Static,
    Const,
}

impl std::fmt::Display for ItemKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ItemKind::Static => f.write_str("static"),
            ItemKind::Const => f.write_str("const"),
        }
    }
}

pub(crate) enum MacroKind {
    ByExample,
    ProcBang,
    ProcDerive,
    ProcAttribute,
}

impl std::fmt::Display for MacroKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MacroKind::ByExample => f.write_str("macro by example"),
            MacroKind::ProcBang => f.write_str("proc macro"),
            MacroKind::ProcDerive => f.write_str("proc macro (derive)"),
            MacroKind::ProcAttribute => f.write_str("proc macro (attribute)"),
        }
    }
}

pub(crate) struct Common {
    pub(crate) id: Id,
    pub(crate) name: String,
    pub(crate) module: String,
    pub(crate) public: bool,
    pub(crate) stability: Option<Stability>,
}

impl Common {
    pub(crate) fn public_str(&self) -> &str {
        match self.public {
            true => "public",
            false => "private",
        }
    }

    pub(crate) fn stable_str(&self) -> &str {
        match &self.stability {
            Some(Stability { stable: true, .. }) => "stable",
            Some(Stability { stable: false, .. }) => "unstable",
            None => "",
        }
    }

    pub(crate) fn feature_str(&self) -> &str {
        match &self.stability {
            Some(Stability { feature, .. }) => &feature,
            None => "",
        }
    }
}

pub(crate) struct Function {
    pub(crate) common: Common,
    pub(crate) kind: FunctionKind,
    pub(crate) impl_: Option<String>,
    pub(crate) trait_id: Option<Id>,
    pub(crate) file: String,
    pub(crate) lines_of_code: usize,
    pub(crate) lines_of_docs: usize,
}

pub(crate) struct Type {
    pub(crate) common: Common,
    pub(crate) kind: TypeKind,
}

pub(crate) struct Trait {
    pub(crate) common: Common,
    pub(crate) implementations: usize,
}

pub(crate) struct Item {
    pub(crate) common: Common,
    pub(crate) kind: ItemKind,
    pub(crate) type_: String,
    pub(crate) value: String,
}

pub(crate) struct Macro {
    pub(crate) common: Common,
    pub(crate) kind: MacroKind,
}

#[derive(Default)]
pub(crate) struct TypeCounters {
    pub(crate) blanket_impls: usize,
    pub(crate) auto_impls: usize,
    pub(crate) trait_impls: usize,
    pub(crate) methods: usize,
    pub(crate) trait_methods: usize,
}

#[derive(Default)]
pub(crate) struct TraitCounters {
    pub(crate) required_methods: usize,
    pub(crate) default_methods: usize,
}

macro_rules! deref_common {
    (impl Deref for $t:ty) => {
        impl std::ops::Deref for $t {
            type Target = Common;

            fn deref(&self) -> &Self::Target {
                &self.common
            }
        }
    };
}

deref_common!(impl Deref for Function);
deref_common!(impl Deref for Type);
deref_common!(impl Deref for Trait);
deref_common!(impl Deref for Item);
deref_common!(impl Deref for Macro);
