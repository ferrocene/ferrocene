use rustdoc_types::{
    Constant, Crate, Enum, Function, GenericBound, Generics, Impl, Import, Item, ItemEnum, Module,
    Primitive, Static, Struct, StructKind, Trait, TraitAlias, Type, Typedef, Union, Variant,
    VariantKind, OpaqueTy, ProcMacro,
};

pub(crate) trait Visitor {
    /* Visitors */

    fn visit_crate(&mut self, crate_: &Crate) {
        self.walk_crate(crate_);
    }

    fn visit_item(&mut self, crate_: &Crate, item: &Item) {
        self.walk_item(crate_, item);
    }

    fn visit_module(&mut self, crate_: &Crate, item: &Item, module: &Module) {
        self.walk_module(crate_, item, module);
    }

    fn visit_import(&mut self, crate_: &Crate, item: &Item, import: &Import) {
        self.walk_import(crate_, item, import);
    }

    fn visit_struct(&mut self, crate_: &Crate, item: &Item, struct_: &Struct) {
        self.walk_struct(crate_, item, struct_);
    }

    fn visit_impl(&mut self, crate_: &Crate, item: &Item, impl_: &Impl) {
        self.walk_impl(crate_, item, impl_);
    }

    fn visit_enum(&mut self, crate_: &Crate, item: &Item, enum_: &Enum) {
        self.walk_enum(crate_, item, enum_);
    }

    fn visit_variant(&mut self, crate_: &Crate, item: &Item, variant: &Variant) {
        self.walk_variant(crate_, item, variant);
    }

    fn visit_trait(&mut self, crate_: &Crate, item: &Item, trait_: &Trait) {
        self.walk_trait(crate_, item, trait_);
    }

    fn visit_union(&mut self, crate_: &Crate, item: &Item, union_: &Union) {
        self.walk_union(crate_, item, union_);
    }

    fn visit_primitive(&mut self, crate_: &Crate, item: &Item, primitive: &Primitive) {
        self.walk_primitive(crate_, item, primitive);
    }

    fn visit_macro(&mut self, _crate_: &Crate, _item: &Item, _macro_: &str) {}
    fn visit_constant(&mut self, _crate_: &Crate, _item: &Item, _constant: &Constant) {}
    fn visit_function(&mut self, _crate_: &Crate, _item: &Item, _function: &Function) {}
    fn visit_struct_field(&mut self, _crate_: &Crate, _item: &Item, _field: &Type) {}
    fn visit_typedef(&mut self, _crate_: &Crate, _item: &Item, _typedef: &Typedef) {}
    fn visit_foreign_type(&mut self, _crate_: &Crate, _item: &Item) {}
    fn visit_trait_alias(&mut self, _crate_: &Crate, _item: &Item, _alias: &TraitAlias) {}
    fn visit_static(&mut self, _crate_: &Crate, _item: &Item, _static_: &Static) {}
    fn visit_opaque_type(&mut self, _crate_: &Crate, _item: &Item, _opaque: &OpaqueTy) {}
    fn visit_proc_macro(&mut self, _crate_: &Crate, _item: &Item, _proc_macro: &ProcMacro) {}

    fn visit_extern_crate(
        &mut self,
        _crate_: &Crate,
        _item: &Item,
        _name: &str,
        _rename: &Option<String>,
    ) {
    }

    fn visit_associated_type(
        &mut self,
        _crate_: &Crate,
        _item: &Item,
        _generics: &Generics,
        _bounds: &[GenericBound],
        _default: &Option<Type>,
    ) {
    }

    fn visit_associated_const(
        &mut self,
        _crate_: &Crate,
        _item: &Item,
        _type_: &Type,
        _default: &Option<String>,
    ) {
    }

    /* Walkers */

    fn walk_crate(&mut self, crate_: &Crate) {
        self.visit_item(crate_, crate_.index.get(&crate_.root).unwrap());
    }

    fn walk_module(&mut self, crate_: &Crate, _item: &Item, module: &Module) {
        for id in &module.items {
            self.visit_item(crate_, crate_.index.get(id).unwrap());
        }
    }

    fn walk_import(&mut self, crate_: &Crate, _item: &Item, import: &Import) {
        if let Some(id) = &import.id {
            self.visit_item(crate_, crate_.index.get(id).unwrap());
        }
    }

    fn walk_struct(&mut self, crate_: &Crate, _item: &Item, struct_: &Struct) {
        for id in &struct_.impls {
            self.visit_item(crate_, crate_.index.get(id).unwrap());
        }
        match &struct_.kind {
            StructKind::Unit => {}
            StructKind::Tuple(items) => {
                for item in items {
                    if let Some(id) = item {
                        self.visit_item(crate_, crate_.index.get(id).unwrap());
                    }
                }
            }
            StructKind::Plain { fields, .. } => {
                for item in fields {
                    self.visit_item(crate_, crate_.index.get(item).unwrap());
                }
            }
        }
    }

    fn walk_impl(&mut self, crate_: &Crate, _item: &Item, impl_: &Impl) {
        for item in &impl_.items {
            self.visit_item(crate_, crate_.index.get(item).unwrap());
        }
    }

    fn walk_enum(&mut self, crate_: &Crate, _item: &Item, enum_: &Enum) {
        for variant in &enum_.variants {
            self.visit_item(crate_, crate_.index.get(variant).unwrap());
        }
        for impl_ in &enum_.impls {
            self.visit_item(crate_, crate_.index.get(impl_).unwrap());
        }
    }

    fn walk_variant(&mut self, crate_: &Crate, _item: &Item, variant: &Variant) {
        match &variant.kind {
            VariantKind::Plain => {}
            VariantKind::Tuple(items) => {
                for item in items {
                    if let Some(id) = item {
                        self.visit_item(crate_, crate_.index.get(id).unwrap());
                    }
                }
            }
            VariantKind::Struct { fields, .. } => {
                for item in fields {
                    self.visit_item(crate_, crate_.index.get(item).unwrap());
                }
            }
        }
    }

    fn walk_trait(&mut self, crate_: &Crate, _item: &Item, trait_: &Trait) {
        for item in &trait_.items {
            self.visit_item(crate_, crate_.index.get(item).unwrap());
        }
        for impl_ in &trait_.implementations {
            self.visit_item(crate_, crate_.index.get(impl_).unwrap());
        }
    }

    fn walk_union(&mut self, crate_: &Crate, _item: &Item, union_: &Union) {
        for field in &union_.fields {
            self.visit_item(crate_, crate_.index.get(field).unwrap());
        }
        for impl_ in &union_.impls {
            self.visit_item(crate_, crate_.index.get(impl_).unwrap());
        }
    }

    fn walk_primitive(&mut self, crate_: &Crate, _item: &Item, primitive: &Primitive) {
        for impl_ in &primitive.impls {
            self.visit_item(crate_, crate_.index.get(impl_).unwrap());
        }
    }

    fn walk_item(&mut self, crate_: &Crate, item: &Item) {
        match &item.inner {
            ItemEnum::Module(module) => self.visit_module(crate_, item, module),
            ItemEnum::ExternCrate { name, rename } => {
                self.visit_extern_crate(crate_, item, name, rename)
            }
            ItemEnum::Import(import) => self.visit_import(crate_, item, import),
            ItemEnum::Union(union_) => self.visit_union(crate_, item, union_),
            ItemEnum::Struct(struct_) => self.visit_struct(crate_, item, struct_),
            ItemEnum::StructField(field) => self.visit_struct_field(crate_, item, field),
            ItemEnum::Enum(enum_) => self.visit_enum(crate_, item, enum_),
            ItemEnum::Variant(variant) => self.visit_variant(crate_, item, variant),
            ItemEnum::Function(function) => self.visit_function(crate_, item, function),
            ItemEnum::Trait(trait_) => self.visit_trait(crate_, item, trait_),
            ItemEnum::TraitAlias(alias) => self.visit_trait_alias(crate_, item, alias),
            ItemEnum::Impl(impl_) => self.visit_impl(crate_, item, impl_),
            ItemEnum::Typedef(typedef) => self.visit_typedef(crate_, item, typedef),
            ItemEnum::OpaqueTy(opaque) => self.visit_opaque_type(crate_, item, opaque),
            ItemEnum::Constant(constant) => self.visit_constant(crate_, item, constant),
            ItemEnum::Static(static_) => self.visit_static(crate_, item, static_),
            ItemEnum::ForeignType => self.visit_foreign_type(crate_, item),
            ItemEnum::Macro(macro_) => self.visit_macro(crate_, item, macro_),
            ItemEnum::ProcMacro(proc_macro) => self.visit_proc_macro(crate_, item, proc_macro),
            ItemEnum::Primitive(primitive) => self.visit_primitive(crate_, item, primitive),
            ItemEnum::AssocConst { type_, default } => {
                self.visit_associated_const(crate_, item, type_, default)
            }
            ItemEnum::AssocType {
                generics,
                bounds,
                default,
            } => self.visit_associated_type(crate_, item, generics, bounds, default),
        }
    }
}
