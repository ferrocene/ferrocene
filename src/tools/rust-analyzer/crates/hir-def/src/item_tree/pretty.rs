//! `ItemTree` debug printer.

use std::fmt::{self, Write};

use la_arena::{Idx, RawIdx};
use span::ErasedFileAstId;

use crate::{
    generics::{TypeOrConstParamData, WherePredicate, WherePredicateTypeTarget},
    item_tree::{
        AttrOwner, Const, DefDatabase, Enum, ExternBlock, ExternCrate, Field, FieldParent,
        FieldsShape, FileItemTreeId, FnFlags, Function, GenericModItem, GenericParams, Impl,
        Interned, ItemTree, Macro2, MacroCall, MacroRules, Mod, ModItem, ModKind, Param, Path,
        RawAttrs, RawVisibilityId, Static, Struct, Trait, TraitAlias, TypeAlias, TypeBound,
        TypeRef, Union, Use, UseTree, UseTreeKind, Variant,
    },
    pretty::{print_path, print_type_bounds, print_type_ref},
    visibility::RawVisibility,
};

pub(super) fn print_item_tree(db: &dyn DefDatabase, tree: &ItemTree) -> String {
    let mut p = Printer { db, tree, buf: String::new(), indent_level: 0, needs_indent: true };

    if let Some(attrs) = tree.attrs.get(&AttrOwner::TopLevel) {
        p.print_attrs(attrs, true, "\n");
    }
    p.blank();

    for item in tree.top_level_items() {
        p.print_mod_item(*item);
    }

    let mut s = p.buf.trim_end_matches('\n').to_owned();
    s.push('\n');
    s
}

macro_rules! w {
    ($dst:expr, $($arg:tt)*) => {
        { let _ = write!($dst, $($arg)*); }
    };
}

macro_rules! wln {
    ($dst:expr) => {
        { let _ = writeln!($dst); }
    };
    ($dst:expr, $($arg:tt)*) => {
        { let _ = writeln!($dst, $($arg)*); }
    };
}

struct Printer<'a> {
    db: &'a dyn DefDatabase,
    tree: &'a ItemTree,
    buf: String,
    indent_level: usize,
    needs_indent: bool,
}

impl Printer<'_> {
    fn indented(&mut self, f: impl FnOnce(&mut Self)) {
        self.indent_level += 1;
        wln!(self);
        f(self);
        self.indent_level -= 1;
        self.buf = self.buf.trim_end_matches('\n').to_owned();
    }

    /// Ensures that a blank line is output before the next text.
    fn blank(&mut self) {
        let mut iter = self.buf.chars().rev().fuse();
        match (iter.next(), iter.next()) {
            (Some('\n'), Some('\n') | None) | (None, None) => {}
            (Some('\n'), Some(_)) => {
                self.buf.push('\n');
            }
            (Some(_), _) => {
                self.buf.push('\n');
                self.buf.push('\n');
            }
            (None, Some(_)) => unreachable!(),
        }
    }

    fn whitespace(&mut self) {
        match self.buf.chars().next_back() {
            None | Some('\n' | ' ') => {}
            _ => self.buf.push(' '),
        }
    }

    fn print_attrs(&mut self, attrs: &RawAttrs, inner: bool, separated_by: &str) {
        let inner = if inner { "!" } else { "" };
        for attr in &**attrs {
            w!(
                self,
                "#{}[{}{}]{}",
                inner,
                attr.path.display(self.db.upcast()),
                attr.input.as_ref().map(|it| it.to_string()).unwrap_or_default(),
                separated_by,
            );
        }
    }

    fn print_attrs_of(&mut self, of: impl Into<AttrOwner>, separated_by: &str) {
        if let Some(attrs) = self.tree.attrs.get(&of.into()) {
            self.print_attrs(attrs, false, separated_by);
        }
    }

    fn print_visibility(&mut self, vis: RawVisibilityId) {
        match &self.tree[vis] {
            RawVisibility::Module(path, _expl) => {
                w!(self, "pub({}) ", path.display(self.db.upcast()))
            }
            RawVisibility::Public => w!(self, "pub "),
        };
    }

    fn print_fields(&mut self, parent: FieldParent, kind: FieldsShape, fields: &[Field]) {
        match kind {
            FieldsShape::Record => {
                self.whitespace();
                w!(self, "{{");
                self.indented(|this| {
                    for (idx, Field { name, type_ref, visibility }) in fields.iter().enumerate() {
                        this.print_attrs_of(
                            AttrOwner::Field(parent, Idx::from_raw(RawIdx::from(idx as u32))),
                            "\n",
                        );
                        this.print_visibility(*visibility);
                        w!(this, "{}: ", name.display(self.db.upcast()));
                        this.print_type_ref(type_ref);
                        wln!(this, ",");
                    }
                });
                w!(self, "}}");
            }
            FieldsShape::Tuple => {
                w!(self, "(");
                self.indented(|this| {
                    for (idx, Field { name, type_ref, visibility }) in fields.iter().enumerate() {
                        this.print_attrs_of(
                            AttrOwner::Field(parent, Idx::from_raw(RawIdx::from(idx as u32))),
                            "\n",
                        );
                        this.print_visibility(*visibility);
                        w!(this, "{}: ", name.display(self.db.upcast()));
                        this.print_type_ref(type_ref);
                        wln!(this, ",");
                    }
                });
                w!(self, ")");
            }
            FieldsShape::Unit => {}
        }
    }

    fn print_fields_and_where_clause(
        &mut self,
        parent: FieldParent,
        kind: FieldsShape,
        fields: &[Field],
        params: &GenericParams,
    ) {
        match kind {
            FieldsShape::Record => {
                if self.print_where_clause(params) {
                    wln!(self);
                }
                self.print_fields(parent, kind, fields);
            }
            FieldsShape::Unit => {
                self.print_where_clause(params);
                self.print_fields(parent, kind, fields);
            }
            FieldsShape::Tuple => {
                self.print_fields(parent, kind, fields);
                self.print_where_clause(params);
            }
        }
    }

    fn print_use_tree(&mut self, use_tree: &UseTree) {
        match &use_tree.kind {
            UseTreeKind::Single { path, alias } => {
                w!(self, "{}", path.display(self.db.upcast()));
                if let Some(alias) = alias {
                    w!(self, " as {}", alias);
                }
            }
            UseTreeKind::Glob { path } => {
                if let Some(path) = path {
                    w!(self, "{}::", path.display(self.db.upcast()));
                }
                w!(self, "*");
            }
            UseTreeKind::Prefixed { prefix, list } => {
                if let Some(prefix) = prefix {
                    w!(self, "{}::", prefix.display(self.db.upcast()));
                }
                w!(self, "{{");
                for (i, tree) in list.iter().enumerate() {
                    if i != 0 {
                        w!(self, ", ");
                    }
                    self.print_use_tree(tree);
                }
                w!(self, "}}");
            }
        }
    }

    fn print_mod_item(&mut self, item: ModItem) {
        self.print_attrs_of(item, "\n");

        match item {
            ModItem::Use(it) => {
                let Use { visibility, use_tree, ast_id } = &self.tree[it];
                self.print_ast_id(ast_id.erase());
                self.print_visibility(*visibility);
                w!(self, "use ");
                self.print_use_tree(use_tree);
                wln!(self, ";");
            }
            ModItem::ExternCrate(it) => {
                let ExternCrate { name, alias, visibility, ast_id } = &self.tree[it];
                self.print_ast_id(ast_id.erase());
                self.print_visibility(*visibility);
                w!(self, "extern crate {}", name.display(self.db.upcast()));
                if let Some(alias) = alias {
                    w!(self, " as {}", alias);
                }
                wln!(self, ";");
            }
            ModItem::ExternBlock(it) => {
                let ExternBlock { abi, ast_id, children } = &self.tree[it];
                self.print_ast_id(ast_id.erase());
                w!(self, "extern ");
                if let Some(abi) = abi {
                    w!(self, "\"{}\" ", abi);
                }
                w!(self, "{{");
                self.indented(|this| {
                    for child in &**children {
                        this.print_mod_item(*child);
                    }
                });
                wln!(self, "}}");
            }
            ModItem::Function(it) => {
                let Function {
                    name,
                    visibility,
                    explicit_generic_params,
                    abi,
                    params,
                    ret_type,
                    ast_id,
                    flags,
                } = &self.tree[it];
                self.print_ast_id(ast_id.erase());
                self.print_visibility(*visibility);
                if flags.contains(FnFlags::HAS_DEFAULT_KW) {
                    w!(self, "default ");
                }
                if flags.contains(FnFlags::HAS_CONST_KW) {
                    w!(self, "const ");
                }
                if flags.contains(FnFlags::HAS_ASYNC_KW) {
                    w!(self, "async ");
                }
                if flags.contains(FnFlags::HAS_UNSAFE_KW) {
                    w!(self, "unsafe ");
                }
                if let Some(abi) = abi {
                    w!(self, "extern \"{}\" ", abi);
                }
                w!(self, "fn {}", name.display(self.db.upcast()));
                self.print_generic_params(explicit_generic_params, it.into());
                w!(self, "(");
                if !params.is_empty() {
                    self.indented(|this| {
                        for (idx, Param { type_ref }) in params.iter().enumerate() {
                            this.print_attrs_of(
                                AttrOwner::Param(it, Idx::from_raw(RawIdx::from(idx as u32))),
                                "\n",
                            );
                            if idx == 0 && flags.contains(FnFlags::HAS_SELF_PARAM) {
                                w!(this, "self: ");
                            }
                            if let Some(type_ref) = type_ref {
                                this.print_type_ref(type_ref);
                            } else {
                                wln!(this, "...");
                            }
                            wln!(this, ",");
                        }
                    });
                }
                w!(self, ") -> ");
                self.print_type_ref(ret_type);
                self.print_where_clause(explicit_generic_params);
                if flags.contains(FnFlags::HAS_BODY) {
                    wln!(self, " {{ ... }}");
                } else {
                    wln!(self, ";");
                }
            }
            ModItem::Struct(it) => {
                let Struct { visibility, name, fields, shape: kind, generic_params, ast_id } =
                    &self.tree[it];
                self.print_ast_id(ast_id.erase());
                self.print_visibility(*visibility);
                w!(self, "struct {}", name.display(self.db.upcast()));
                self.print_generic_params(generic_params, it.into());
                self.print_fields_and_where_clause(
                    FieldParent::Struct(it),
                    *kind,
                    fields,
                    generic_params,
                );
                if matches!(kind, FieldsShape::Record) {
                    wln!(self);
                } else {
                    wln!(self, ";");
                }
            }
            ModItem::Union(it) => {
                let Union { name, visibility, fields, generic_params, ast_id } = &self.tree[it];
                self.print_ast_id(ast_id.erase());
                self.print_visibility(*visibility);
                w!(self, "union {}", name.display(self.db.upcast()));
                self.print_generic_params(generic_params, it.into());
                self.print_fields_and_where_clause(
                    FieldParent::Union(it),
                    FieldsShape::Record,
                    fields,
                    generic_params,
                );
                wln!(self);
            }
            ModItem::Enum(it) => {
                let Enum { name, visibility, variants, generic_params, ast_id } = &self.tree[it];
                self.print_ast_id(ast_id.erase());
                self.print_visibility(*visibility);
                w!(self, "enum {}", name.display(self.db.upcast()));
                self.print_generic_params(generic_params, it.into());
                self.print_where_clause_and_opening_brace(generic_params);
                self.indented(|this| {
                    for variant in FileItemTreeId::range_iter(variants.clone()) {
                        let Variant { name, fields, shape: kind, ast_id } = &this.tree[variant];
                        this.print_ast_id(ast_id.erase());
                        this.print_attrs_of(variant, "\n");
                        w!(this, "{}", name.display(self.db.upcast()));
                        this.print_fields(FieldParent::Variant(variant), *kind, fields);
                        wln!(this, ",");
                    }
                });
                wln!(self, "}}");
            }
            ModItem::Const(it) => {
                let Const { name, visibility, type_ref, ast_id, has_body: _ } = &self.tree[it];
                self.print_ast_id(ast_id.erase());
                self.print_visibility(*visibility);
                w!(self, "const ");
                match name {
                    Some(name) => w!(self, "{}", name.display(self.db.upcast())),
                    None => w!(self, "_"),
                }
                w!(self, ": ");
                self.print_type_ref(type_ref);
                wln!(self, " = _;");
            }
            ModItem::Static(it) => {
                let Static { name, visibility, mutable, type_ref, ast_id } = &self.tree[it];
                self.print_ast_id(ast_id.erase());
                self.print_visibility(*visibility);
                w!(self, "static ");
                if *mutable {
                    w!(self, "mut ");
                }
                w!(self, "{}: ", name.display(self.db.upcast()));
                self.print_type_ref(type_ref);
                w!(self, " = _;");
                wln!(self);
            }
            ModItem::Trait(it) => {
                let Trait { name, visibility, is_auto, is_unsafe, items, generic_params, ast_id } =
                    &self.tree[it];
                self.print_ast_id(ast_id.erase());
                self.print_visibility(*visibility);
                if *is_unsafe {
                    w!(self, "unsafe ");
                }
                if *is_auto {
                    w!(self, "auto ");
                }
                w!(self, "trait {}", name.display(self.db.upcast()));
                self.print_generic_params(generic_params, it.into());
                self.print_where_clause_and_opening_brace(generic_params);
                self.indented(|this| {
                    for item in &**items {
                        this.print_mod_item((*item).into());
                    }
                });
                wln!(self, "}}");
            }
            ModItem::TraitAlias(it) => {
                let TraitAlias { name, visibility, generic_params, ast_id } = &self.tree[it];
                self.print_ast_id(ast_id.erase());
                self.print_visibility(*visibility);
                w!(self, "trait {}", name.display(self.db.upcast()));
                self.print_generic_params(generic_params, it.into());
                w!(self, " = ");
                self.print_where_clause(generic_params);
                w!(self, ";");
                wln!(self);
            }
            ModItem::Impl(it) => {
                let Impl {
                    target_trait,
                    self_ty,
                    is_negative,
                    is_unsafe,
                    items,
                    generic_params,
                    ast_id,
                } = &self.tree[it];
                self.print_ast_id(ast_id.erase());
                if *is_unsafe {
                    w!(self, "unsafe");
                }
                w!(self, "impl");
                self.print_generic_params(generic_params, it.into());
                w!(self, " ");
                if *is_negative {
                    w!(self, "!");
                }
                if let Some(tr) = target_trait {
                    self.print_path(&tr.path);
                    w!(self, " for ");
                }
                self.print_type_ref(self_ty);
                self.print_where_clause_and_opening_brace(generic_params);
                self.indented(|this| {
                    for item in &**items {
                        this.print_mod_item((*item).into());
                    }
                });
                wln!(self, "}}");
            }
            ModItem::TypeAlias(it) => {
                let TypeAlias { name, visibility, bounds, type_ref, generic_params, ast_id } =
                    &self.tree[it];
                self.print_ast_id(ast_id.erase());
                self.print_visibility(*visibility);
                w!(self, "type {}", name.display(self.db.upcast()));
                self.print_generic_params(generic_params, it.into());
                if !bounds.is_empty() {
                    w!(self, ": ");
                    self.print_type_bounds(bounds);
                }
                if let Some(ty) = type_ref {
                    w!(self, " = ");
                    self.print_type_ref(ty);
                }
                self.print_where_clause(generic_params);
                w!(self, ";");
                wln!(self);
            }
            ModItem::Mod(it) => {
                let Mod { name, visibility, kind, ast_id } = &self.tree[it];
                self.print_ast_id(ast_id.erase());
                self.print_visibility(*visibility);
                w!(self, "mod {}", name.display(self.db.upcast()));
                match kind {
                    ModKind::Inline { items } => {
                        w!(self, " {{");
                        self.indented(|this| {
                            for item in &**items {
                                this.print_mod_item(*item);
                            }
                        });
                        wln!(self, "}}");
                    }
                    ModKind::Outline => {
                        wln!(self, ";");
                    }
                }
            }
            ModItem::MacroCall(it) => {
                let MacroCall { path, ast_id, expand_to, ctxt } = &self.tree[it];
                let _ = writeln!(
                    self,
                    "// AstId: {:?}, SyntaxContext: {}, ExpandTo: {:?}",
                    ast_id.erase().into_raw(),
                    ctxt,
                    expand_to
                );
                wln!(self, "{}!(...);", path.display(self.db.upcast()));
            }
            ModItem::MacroRules(it) => {
                let MacroRules { name, ast_id } = &self.tree[it];
                self.print_ast_id(ast_id.erase());
                wln!(self, "macro_rules! {} {{ ... }}", name.display(self.db.upcast()));
            }
            ModItem::Macro2(it) => {
                let Macro2 { name, visibility, ast_id } = &self.tree[it];
                self.print_ast_id(ast_id.erase());
                self.print_visibility(*visibility);
                wln!(self, "macro {} {{ ... }}", name.display(self.db.upcast()));
            }
        }

        self.blank();
    }

    fn print_type_ref(&mut self, type_ref: &TypeRef) {
        print_type_ref(self.db, type_ref, self).unwrap();
    }

    fn print_type_bounds(&mut self, bounds: &[Interned<TypeBound>]) {
        print_type_bounds(self.db, bounds, self).unwrap();
    }

    fn print_path(&mut self, path: &Path) {
        print_path(self.db, path, self).unwrap();
    }

    fn print_generic_params(&mut self, params: &GenericParams, parent: GenericModItem) {
        if params.is_empty() {
            return;
        }

        w!(self, "<");
        let mut first = true;
        for (idx, lt) in params.iter_lt() {
            if !first {
                w!(self, ", ");
            }
            first = false;
            self.print_attrs_of(AttrOwner::LifetimeParamData(parent, idx), " ");
            w!(self, "{}", lt.name.display(self.db.upcast()));
        }
        for (idx, x) in params.iter_type_or_consts() {
            if !first {
                w!(self, ", ");
            }
            first = false;
            self.print_attrs_of(AttrOwner::TypeOrConstParamData(parent, idx), " ");
            match x {
                TypeOrConstParamData::TypeParamData(ty) => match &ty.name {
                    Some(name) => w!(self, "{}", name.display(self.db.upcast())),
                    None => w!(self, "_anon_{}", idx.into_raw()),
                },
                TypeOrConstParamData::ConstParamData(konst) => {
                    w!(self, "const {}: ", konst.name.display(self.db.upcast()));
                    self.print_type_ref(&konst.ty);
                }
            }
        }
        w!(self, ">");
    }

    fn print_where_clause_and_opening_brace(&mut self, params: &GenericParams) {
        if self.print_where_clause(params) {
            w!(self, "\n{{");
        } else {
            self.whitespace();
            w!(self, "{{");
        }
    }

    fn print_where_clause(&mut self, params: &GenericParams) -> bool {
        if params.where_predicates().next().is_none() {
            return false;
        }

        w!(self, "\nwhere");
        self.indented(|this| {
            for (i, pred) in params.where_predicates().enumerate() {
                if i != 0 {
                    wln!(this, ",");
                }

                let (target, bound) = match pred {
                    WherePredicate::TypeBound { target, bound } => (target, bound),
                    WherePredicate::Lifetime { target, bound } => {
                        wln!(
                            this,
                            "{}: {},",
                            target.name.display(self.db.upcast()),
                            bound.name.display(self.db.upcast())
                        );
                        continue;
                    }
                    WherePredicate::ForLifetime { lifetimes, target, bound } => {
                        w!(this, "for<");
                        for (i, lt) in lifetimes.iter().enumerate() {
                            if i != 0 {
                                w!(this, ", ");
                            }
                            w!(this, "{}", lt.display(self.db.upcast()));
                        }
                        w!(this, "> ");
                        (target, bound)
                    }
                };

                match target {
                    WherePredicateTypeTarget::TypeRef(ty) => this.print_type_ref(ty),
                    WherePredicateTypeTarget::TypeOrConstParam(id) => match params[*id].name() {
                        Some(name) => w!(this, "{}", name.display(self.db.upcast())),
                        None => w!(this, "_anon_{}", id.into_raw()),
                    },
                }
                w!(this, ": ");
                this.print_type_bounds(std::slice::from_ref(bound));
            }
        });
        true
    }

    fn print_ast_id(&mut self, ast_id: ErasedFileAstId) {
        wln!(self, "// AstId: {:?}", ast_id.into_raw());
    }
}

impl Write for Printer<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for line in s.split_inclusive('\n') {
            if self.needs_indent {
                match self.buf.chars().last() {
                    Some('\n') | None => {}
                    _ => self.buf.push('\n'),
                }
                self.buf.push_str(&"    ".repeat(self.indent_level));
                self.needs_indent = false;
            }

            self.buf.push_str(line);
            self.needs_indent = line.ends_with('\n');
        }

        Ok(())
    }
}
