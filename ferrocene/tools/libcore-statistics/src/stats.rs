use crate::render::render_impl;
use crate::stability::{parse_stability, Stability};
use crate::visitor::Visitor;
use std::collections::HashSet;

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

pub(crate) struct Function {
    pub(crate) name: String,
    pub(crate) module: String,
    pub(crate) kind: FunctionKind,
    pub(crate) impl_: Option<String>,
    pub(crate) public: bool,
    pub(crate) stability: Option<Stability>,
}

pub(crate) struct StatsCollector {
    seen: HashSet<rustdoc_types::Id>,
    name_stack: Vec<String>,
    module_stack: Vec<String>,
    stability_stack: Vec<Stability>,
    inside: Inside,
    pub(crate) functions: Vec<Function>,
}

impl StatsCollector {
    pub(crate) fn new() -> Self {
        Self {
            seen: HashSet::new(),
            name_stack: Vec::new(),
            module_stack: Vec::new(),
            stability_stack: Vec::new(),
            inside: Inside::None,
            functions: Vec::new(),
        }
    }

    fn override_inside(&mut self, f: impl FnOnce(&mut Self)) {
        let old = std::mem::take(&mut self.inside);
        f(self);
        self.inside = old;
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
                    signature,
                    public: trait_.visibility == rustdoc_types::Visibility::Public,
                };
            } else {
                this.inside = Inside::ItemImpl { signature };
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
                public: item.visibility == rustdoc_types::Visibility::Public,
            };
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
        self.functions.push(Function {
            name: self.name_stack.join("::"),
            module: self.module_stack.join("::"),
            kind: match self.inside {
                Inside::None => FunctionKind::Function,
                Inside::ItemImpl { .. } => FunctionKind::Method,
                Inside::TraitImpl { .. } => FunctionKind::TraitMethod,
                Inside::TraitDefinition { .. } => FunctionKind::TraitMethodDefinition {
                    has_default: function.has_body,
                },
            },
            impl_: match &self.inside {
                Inside::ItemImpl { signature } => Some(signature.clone()),
                Inside::TraitImpl { signature, .. } => Some(signature.clone()),
                _ => None,
            },
            public: match &self.inside {
                Inside::TraitImpl { public, .. } => *public,
                Inside::TraitDefinition { public } => *public,
                _ => item.visibility == rustdoc_types::Visibility::Public,
            },
            stability: self.stability_stack.last().cloned(),
        });
    }
}

#[derive(Default)]
enum Inside {
    #[default]
    None,
    ItemImpl {
        signature: String,
    },
    TraitImpl {
        signature: String,
        public: bool,
    },
    TraitDefinition {
        public: bool,
    },
}
