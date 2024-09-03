//! Many kinds of items or constructs can have generic parameters: functions,
//! structs, impls, traits, etc. This module provides a common HIR for these
//! generic parameters. See also the `Generics` type and the `generics_of` query
//! in rustc.

use std::ops;

use either::Either;
use hir_expand::{
    name::{AsName, Name},
    ExpandResult,
};
use intern::Interned;
use la_arena::{Arena, RawIdx};
use stdx::impl_from;
use syntax::ast::{self, HasGenericParams, HasName, HasTypeBounds};
use triomphe::Arc;

use crate::{
    db::DefDatabase,
    expander::Expander,
    item_tree::{AttrOwner, FileItemTreeId, GenericModItem, GenericsItemTreeNode, ItemTree},
    lower::LowerCtx,
    nameres::{DefMap, MacroSubNs},
    type_ref::{ConstRef, LifetimeRef, TypeBound, TypeRef},
    AdtId, ConstParamId, GenericDefId, HasModule, ItemTreeLoc, LifetimeParamId,
    LocalLifetimeParamId, LocalTypeOrConstParamId, Lookup, TypeOrConstParamId, TypeParamId,
};

/// The index of the self param in the generic of the non-parent definition.
const SELF_PARAM_ID_IN_SELF: la_arena::Idx<TypeOrConstParamData> =
    LocalTypeOrConstParamId::from_raw(RawIdx::from_u32(0));

/// Data about a generic type parameter (to a function, struct, impl, ...).
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct TypeParamData {
    /// [`None`] only if the type ref is an [`TypeRef::ImplTrait`]. FIXME: Might be better to just
    /// make it always be a value, giving impl trait a special name.
    pub name: Option<Name>,
    pub default: Option<Interned<TypeRef>>,
    pub provenance: TypeParamProvenance,
}

/// Data about a generic lifetime parameter (to a function, struct, impl, ...).
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct LifetimeParamData {
    pub name: Name,
}

/// Data about a generic const parameter (to a function, struct, impl, ...).
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct ConstParamData {
    pub name: Name,
    pub ty: Interned<TypeRef>,
    pub default: Option<ConstRef>,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum TypeParamProvenance {
    TypeParamList,
    TraitSelf,
    ArgumentImplTrait,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum TypeOrConstParamData {
    TypeParamData(TypeParamData),
    ConstParamData(ConstParamData),
}

impl TypeOrConstParamData {
    pub fn name(&self) -> Option<&Name> {
        match self {
            TypeOrConstParamData::TypeParamData(it) => it.name.as_ref(),
            TypeOrConstParamData::ConstParamData(it) => Some(&it.name),
        }
    }

    pub fn has_default(&self) -> bool {
        match self {
            TypeOrConstParamData::TypeParamData(it) => it.default.is_some(),
            TypeOrConstParamData::ConstParamData(it) => it.default.is_some(),
        }
    }

    pub fn type_param(&self) -> Option<&TypeParamData> {
        match self {
            TypeOrConstParamData::TypeParamData(it) => Some(it),
            TypeOrConstParamData::ConstParamData(_) => None,
        }
    }

    pub fn const_param(&self) -> Option<&ConstParamData> {
        match self {
            TypeOrConstParamData::TypeParamData(_) => None,
            TypeOrConstParamData::ConstParamData(it) => Some(it),
        }
    }

    pub fn is_trait_self(&self) -> bool {
        match self {
            TypeOrConstParamData::TypeParamData(it) => {
                it.provenance == TypeParamProvenance::TraitSelf
            }
            TypeOrConstParamData::ConstParamData(_) => false,
        }
    }
}

impl_from!(TypeParamData, ConstParamData for TypeOrConstParamData);

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum GenericParamData {
    TypeParamData(TypeParamData),
    ConstParamData(ConstParamData),
    LifetimeParamData(LifetimeParamData),
}

impl GenericParamData {
    pub fn name(&self) -> Option<&Name> {
        match self {
            GenericParamData::TypeParamData(it) => it.name.as_ref(),
            GenericParamData::ConstParamData(it) => Some(&it.name),
            GenericParamData::LifetimeParamData(it) => Some(&it.name),
        }
    }

    pub fn type_param(&self) -> Option<&TypeParamData> {
        match self {
            GenericParamData::TypeParamData(it) => Some(it),
            _ => None,
        }
    }

    pub fn const_param(&self) -> Option<&ConstParamData> {
        match self {
            GenericParamData::ConstParamData(it) => Some(it),
            _ => None,
        }
    }

    pub fn lifetime_param(&self) -> Option<&LifetimeParamData> {
        match self {
            GenericParamData::LifetimeParamData(it) => Some(it),
            _ => None,
        }
    }
}

impl_from!(TypeParamData, ConstParamData, LifetimeParamData for GenericParamData);

pub enum GenericParamDataRef<'a> {
    TypeParamData(&'a TypeParamData),
    ConstParamData(&'a ConstParamData),
    LifetimeParamData(&'a LifetimeParamData),
}

/// Data about the generic parameters of a function, struct, impl, etc.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub struct GenericParams {
    type_or_consts: Arena<TypeOrConstParamData>,
    lifetimes: Arena<LifetimeParamData>,
    where_predicates: Box<[WherePredicate]>,
}

impl ops::Index<LocalTypeOrConstParamId> for GenericParams {
    type Output = TypeOrConstParamData;
    fn index(&self, index: LocalTypeOrConstParamId) -> &TypeOrConstParamData {
        &self.type_or_consts[index]
    }
}

impl ops::Index<LocalLifetimeParamId> for GenericParams {
    type Output = LifetimeParamData;
    fn index(&self, index: LocalLifetimeParamId) -> &LifetimeParamData {
        &self.lifetimes[index]
    }
}

/// A single predicate from a where clause, i.e. `where Type: Trait`. Combined
/// where clauses like `where T: Foo + Bar` are turned into multiple of these.
/// It might still result in multiple actual predicates though, because of
/// associated type bindings like `Iterator<Item = u32>`.
#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum WherePredicate {
    TypeBound {
        target: WherePredicateTypeTarget,
        bound: Interned<TypeBound>,
    },
    Lifetime {
        target: LifetimeRef,
        bound: LifetimeRef,
    },
    ForLifetime {
        lifetimes: Box<[Name]>,
        target: WherePredicateTypeTarget,
        bound: Interned<TypeBound>,
    },
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum WherePredicateTypeTarget {
    TypeRef(Interned<TypeRef>),
    /// For desugared where predicates that can directly refer to a type param.
    TypeOrConstParam(LocalTypeOrConstParamId),
}

impl GenericParams {
    /// Number of Generic parameters (type_or_consts + lifetimes)
    #[inline]
    pub fn len(&self) -> usize {
        self.type_or_consts.len() + self.lifetimes.len()
    }

    #[inline]
    pub fn len_lifetimes(&self) -> usize {
        self.lifetimes.len()
    }

    #[inline]
    pub fn len_type_or_consts(&self) -> usize {
        self.type_or_consts.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn where_predicates(&self) -> std::slice::Iter<'_, WherePredicate> {
        self.where_predicates.iter()
    }

    /// Iterator of type_or_consts field
    #[inline]
    pub fn iter_type_or_consts(
        &self,
    ) -> impl DoubleEndedIterator<Item = (LocalTypeOrConstParamId, &TypeOrConstParamData)> {
        self.type_or_consts.iter()
    }

    /// Iterator of lifetimes field
    #[inline]
    pub fn iter_lt(
        &self,
    ) -> impl DoubleEndedIterator<Item = (LocalLifetimeParamId, &LifetimeParamData)> {
        self.lifetimes.iter()
    }

    pub fn find_type_by_name(&self, name: &Name, parent: GenericDefId) -> Option<TypeParamId> {
        self.type_or_consts.iter().find_map(|(id, p)| {
            if p.name().as_ref() == Some(&name) && p.type_param().is_some() {
                Some(TypeParamId::from_unchecked(TypeOrConstParamId { local_id: id, parent }))
            } else {
                None
            }
        })
    }

    pub fn find_const_by_name(&self, name: &Name, parent: GenericDefId) -> Option<ConstParamId> {
        self.type_or_consts.iter().find_map(|(id, p)| {
            if p.name().as_ref() == Some(&name) && p.const_param().is_some() {
                Some(ConstParamId::from_unchecked(TypeOrConstParamId { local_id: id, parent }))
            } else {
                None
            }
        })
    }

    #[inline]
    pub fn trait_self_param(&self) -> Option<LocalTypeOrConstParamId> {
        if self.type_or_consts.is_empty() {
            return None;
        }
        matches!(
            self.type_or_consts[SELF_PARAM_ID_IN_SELF],
            TypeOrConstParamData::TypeParamData(TypeParamData {
                provenance: TypeParamProvenance::TraitSelf,
                ..
            })
        )
        .then(|| SELF_PARAM_ID_IN_SELF)
    }

    pub fn find_lifetime_by_name(
        &self,
        name: &Name,
        parent: GenericDefId,
    ) -> Option<LifetimeParamId> {
        self.lifetimes.iter().find_map(|(id, p)| {
            if &p.name == name {
                Some(LifetimeParamId { local_id: id, parent })
            } else {
                None
            }
        })
    }

    pub(crate) fn generic_params_query(
        db: &dyn DefDatabase,
        def: GenericDefId,
    ) -> Interned<GenericParams> {
        let _p = tracing::info_span!("generic_params_query").entered();

        let krate = def.krate(db);
        let cfg_options = db.crate_graph();
        let cfg_options = &cfg_options[krate].cfg_options;

        // Returns the generic parameters that are enabled under the current `#[cfg]` options
        let enabled_params =
            |params: &Interned<GenericParams>, item_tree: &ItemTree, parent: GenericModItem| {
                let enabled = |param| item_tree.attrs(db, krate, param).is_cfg_enabled(cfg_options);
                let attr_owner_ct = |param| AttrOwner::TypeOrConstParamData(parent, param);
                let attr_owner_lt = |param| AttrOwner::LifetimeParamData(parent, param);

                // In the common case, no parameters will by disabled by `#[cfg]` attributes.
                // Therefore, make a first pass to check if all parameters are enabled and, if so,
                // clone the `Interned<GenericParams>` instead of recreating an identical copy.
                let all_type_or_consts_enabled =
                    params.type_or_consts.iter().all(|(idx, _)| enabled(attr_owner_ct(idx)));
                let all_lifetimes_enabled =
                    params.lifetimes.iter().all(|(idx, _)| enabled(attr_owner_lt(idx)));

                if all_type_or_consts_enabled && all_lifetimes_enabled {
                    params.clone()
                } else {
                    Interned::new(GenericParams {
                        type_or_consts: all_type_or_consts_enabled
                            .then(|| params.type_or_consts.clone())
                            .unwrap_or_else(|| {
                                params
                                    .type_or_consts
                                    .iter()
                                    .filter(|&(idx, _)| enabled(attr_owner_ct(idx)))
                                    .map(|(_, param)| param.clone())
                                    .collect()
                            }),
                        lifetimes: all_lifetimes_enabled
                            .then(|| params.lifetimes.clone())
                            .unwrap_or_else(|| {
                                params
                                    .lifetimes
                                    .iter()
                                    .filter(|&(idx, _)| enabled(attr_owner_lt(idx)))
                                    .map(|(_, param)| param.clone())
                                    .collect()
                            }),
                        where_predicates: params.where_predicates.clone(),
                    })
                }
            };
        fn id_to_generics<Id: GenericsItemTreeNode>(
            db: &dyn DefDatabase,
            id: impl for<'db> Lookup<
                Database<'db> = dyn DefDatabase + 'db,
                Data = impl ItemTreeLoc<Id = Id>,
            >,
            enabled_params: impl Fn(
                &Interned<GenericParams>,
                &ItemTree,
                GenericModItem,
            ) -> Interned<GenericParams>,
        ) -> Interned<GenericParams>
        where
            FileItemTreeId<Id>: Into<GenericModItem>,
        {
            let id = id.lookup(db).item_tree_id();
            let tree = id.item_tree(db);
            let item = &tree[id.value];
            enabled_params(item.generic_params(), &tree, id.value.into())
        }

        match def {
            GenericDefId::FunctionId(id) => {
                let loc = id.lookup(db);
                let tree = loc.id.item_tree(db);
                let item = &tree[loc.id.value];

                let enabled_params =
                    enabled_params(&item.explicit_generic_params, &tree, loc.id.value.into());

                let module = loc.container.module(db);
                let func_data = db.function_data(id);
                if func_data.params.is_empty() {
                    enabled_params
                } else {
                    let mut generic_params = GenericParamsCollector {
                        type_or_consts: enabled_params.type_or_consts.clone(),
                        lifetimes: enabled_params.lifetimes.clone(),
                        where_predicates: enabled_params.where_predicates.clone().into(),
                    };

                    // Don't create an `Expander` if not needed since this
                    // could cause a reparse after the `ItemTree` has been created due to the spanmap.
                    let mut expander = None;
                    for param in func_data.params.iter() {
                        generic_params.fill_implicit_impl_trait_args(
                            db,
                            &mut expander,
                            &mut || {
                                (module.def_map(db), Expander::new(db, loc.id.file_id(), module))
                            },
                            param,
                        );
                    }
                    Interned::new(generic_params.finish())
                }
            }
            GenericDefId::AdtId(AdtId::StructId(id)) => id_to_generics(db, id, enabled_params),
            GenericDefId::AdtId(AdtId::EnumId(id)) => id_to_generics(db, id, enabled_params),
            GenericDefId::AdtId(AdtId::UnionId(id)) => id_to_generics(db, id, enabled_params),
            GenericDefId::TraitId(id) => id_to_generics(db, id, enabled_params),
            GenericDefId::TraitAliasId(id) => id_to_generics(db, id, enabled_params),
            GenericDefId::TypeAliasId(id) => id_to_generics(db, id, enabled_params),
            GenericDefId::ImplId(id) => id_to_generics(db, id, enabled_params),
            GenericDefId::ConstId(_) => Interned::new(GenericParams {
                type_or_consts: Default::default(),
                lifetimes: Default::default(),
                where_predicates: Default::default(),
            }),
        }
    }
}

#[derive(Clone, Default)]
pub(crate) struct GenericParamsCollector {
    pub(crate) type_or_consts: Arena<TypeOrConstParamData>,
    lifetimes: Arena<LifetimeParamData>,
    where_predicates: Vec<WherePredicate>,
}

impl GenericParamsCollector {
    pub(crate) fn fill(
        &mut self,
        lower_ctx: &LowerCtx<'_>,
        node: &dyn HasGenericParams,
        add_param_attrs: impl FnMut(
            Either<LocalTypeOrConstParamId, LocalLifetimeParamId>,
            ast::GenericParam,
        ),
    ) {
        if let Some(params) = node.generic_param_list() {
            self.fill_params(lower_ctx, params, add_param_attrs)
        }
        if let Some(where_clause) = node.where_clause() {
            self.fill_where_predicates(lower_ctx, where_clause);
        }
    }

    pub(crate) fn fill_bounds(
        &mut self,
        lower_ctx: &LowerCtx<'_>,
        type_bounds: Option<ast::TypeBoundList>,
        target: Either<TypeRef, LifetimeRef>,
    ) {
        for bound in type_bounds.iter().flat_map(|type_bound_list| type_bound_list.bounds()) {
            self.add_where_predicate_from_bound(lower_ctx, bound, None, target.clone());
        }
    }

    fn fill_params(
        &mut self,
        lower_ctx: &LowerCtx<'_>,
        params: ast::GenericParamList,
        mut add_param_attrs: impl FnMut(
            Either<LocalTypeOrConstParamId, LocalLifetimeParamId>,
            ast::GenericParam,
        ),
    ) {
        for type_or_const_param in params.type_or_const_params() {
            match type_or_const_param {
                ast::TypeOrConstParam::Type(type_param) => {
                    let name = type_param.name().map_or_else(Name::missing, |it| it.as_name());
                    // FIXME: Use `Path::from_src`
                    let default = type_param
                        .default_type()
                        .map(|it| Interned::new(TypeRef::from_ast(lower_ctx, it)));
                    let param = TypeParamData {
                        name: Some(name.clone()),
                        default,
                        provenance: TypeParamProvenance::TypeParamList,
                    };
                    let idx = self.type_or_consts.alloc(param.into());
                    let type_ref = TypeRef::Path(name.into());
                    self.fill_bounds(
                        lower_ctx,
                        type_param.type_bound_list(),
                        Either::Left(type_ref),
                    );
                    add_param_attrs(Either::Left(idx), ast::GenericParam::TypeParam(type_param));
                }
                ast::TypeOrConstParam::Const(const_param) => {
                    let name = const_param.name().map_or_else(Name::missing, |it| it.as_name());
                    let ty = const_param
                        .ty()
                        .map_or(TypeRef::Error, |it| TypeRef::from_ast(lower_ctx, it));
                    let param = ConstParamData {
                        name,
                        ty: Interned::new(ty),
                        default: ConstRef::from_const_param(lower_ctx, &const_param),
                    };
                    let idx = self.type_or_consts.alloc(param.into());
                    add_param_attrs(Either::Left(idx), ast::GenericParam::ConstParam(const_param));
                }
            }
        }
        for lifetime_param in params.lifetime_params() {
            let name =
                lifetime_param.lifetime().map_or_else(Name::missing, |lt| Name::new_lifetime(&lt));
            let param = LifetimeParamData { name: name.clone() };
            let idx = self.lifetimes.alloc(param);
            let lifetime_ref = LifetimeRef::new_name(name);
            self.fill_bounds(
                lower_ctx,
                lifetime_param.type_bound_list(),
                Either::Right(lifetime_ref),
            );
            add_param_attrs(Either::Right(idx), ast::GenericParam::LifetimeParam(lifetime_param));
        }
    }

    fn fill_where_predicates(&mut self, lower_ctx: &LowerCtx<'_>, where_clause: ast::WhereClause) {
        for pred in where_clause.predicates() {
            let target = if let Some(type_ref) = pred.ty() {
                Either::Left(TypeRef::from_ast(lower_ctx, type_ref))
            } else if let Some(lifetime) = pred.lifetime() {
                Either::Right(LifetimeRef::new(&lifetime))
            } else {
                continue;
            };

            let lifetimes: Option<Box<_>> = pred.generic_param_list().map(|param_list| {
                // Higher-Ranked Trait Bounds
                param_list
                    .lifetime_params()
                    .map(|lifetime_param| {
                        lifetime_param
                            .lifetime()
                            .map_or_else(Name::missing, |lt| Name::new_lifetime(&lt))
                    })
                    .collect()
            });
            for bound in pred.type_bound_list().iter().flat_map(|l| l.bounds()) {
                self.add_where_predicate_from_bound(
                    lower_ctx,
                    bound,
                    lifetimes.as_deref(),
                    target.clone(),
                );
            }
        }
    }

    fn add_where_predicate_from_bound(
        &mut self,
        lower_ctx: &LowerCtx<'_>,
        bound: ast::TypeBound,
        hrtb_lifetimes: Option<&[Name]>,
        target: Either<TypeRef, LifetimeRef>,
    ) {
        let bound = TypeBound::from_ast(lower_ctx, bound);
        self.fill_impl_trait_bounds(lower_ctx.take_impl_traits_bounds());
        let predicate = match (target, bound) {
            (Either::Left(type_ref), bound) => match hrtb_lifetimes {
                Some(hrtb_lifetimes) => WherePredicate::ForLifetime {
                    lifetimes: hrtb_lifetimes.to_vec().into_boxed_slice(),
                    target: WherePredicateTypeTarget::TypeRef(Interned::new(type_ref)),
                    bound: Interned::new(bound),
                },
                None => WherePredicate::TypeBound {
                    target: WherePredicateTypeTarget::TypeRef(Interned::new(type_ref)),
                    bound: Interned::new(bound),
                },
            },
            (Either::Right(lifetime), TypeBound::Lifetime(bound)) => {
                WherePredicate::Lifetime { target: lifetime, bound }
            }
            _ => return,
        };
        self.where_predicates.push(predicate);
    }

    fn fill_impl_trait_bounds(&mut self, impl_bounds: Vec<Vec<Interned<TypeBound>>>) {
        for bounds in impl_bounds {
            let param = TypeParamData {
                name: None,
                default: None,
                provenance: TypeParamProvenance::ArgumentImplTrait,
            };
            let param_id = self.type_or_consts.alloc(param.into());
            for bound in bounds {
                self.where_predicates.push(WherePredicate::TypeBound {
                    target: WherePredicateTypeTarget::TypeOrConstParam(param_id),
                    bound,
                });
            }
        }
    }

    fn fill_implicit_impl_trait_args(
        &mut self,
        db: &dyn DefDatabase,
        // FIXME: Change this back to `LazyCell` if https://github.com/rust-lang/libs-team/issues/429 is accepted.
        exp: &mut Option<(Arc<DefMap>, Expander)>,
        exp_fill: &mut dyn FnMut() -> (Arc<DefMap>, Expander),
        type_ref: &TypeRef,
    ) {
        type_ref.walk(&mut |type_ref| {
            if let TypeRef::ImplTrait(bounds) = type_ref {
                let param = TypeParamData {
                    name: None,
                    default: None,
                    provenance: TypeParamProvenance::ArgumentImplTrait,
                };
                let param_id = self.type_or_consts.alloc(param.into());
                for bound in bounds {
                    self.where_predicates.push(WherePredicate::TypeBound {
                        target: WherePredicateTypeTarget::TypeOrConstParam(param_id),
                        bound: bound.clone(),
                    });
                }
            }
            if let TypeRef::Macro(mc) = type_ref {
                let macro_call = mc.to_node(db.upcast());
                let (def_map, expander) = exp.get_or_insert_with(&mut *exp_fill);

                let module = expander.module.local_id;
                let resolver = |path: &_| {
                    def_map
                        .resolve_path(
                            db,
                            module,
                            path,
                            crate::item_scope::BuiltinShadowMode::Other,
                            Some(MacroSubNs::Bang),
                        )
                        .0
                        .take_macros()
                };
                if let Ok(ExpandResult { value: Some((mark, expanded)), .. }) =
                    expander.enter_expand(db, macro_call, resolver)
                {
                    let ctx = expander.ctx(db);
                    let type_ref = TypeRef::from_ast(&ctx, expanded.tree());
                    self.fill_implicit_impl_trait_args(db, &mut *exp, exp_fill, &type_ref);
                    exp.get_or_insert_with(&mut *exp_fill).1.exit(mark);
                }
            }
        });
    }

    pub(crate) fn finish(self) -> GenericParams {
        let Self { mut lifetimes, mut type_or_consts, where_predicates } = self;
        lifetimes.shrink_to_fit();
        type_or_consts.shrink_to_fit();
        GenericParams {
            type_or_consts,
            lifetimes,
            where_predicates: where_predicates.into_boxed_slice(),
        }
    }
}
