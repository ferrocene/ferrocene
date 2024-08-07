use ast::ptr::P;
use ast::HasAttrs;
use rustc_ast::mut_visit::MutVisitor;
use rustc_ast::visit::BoundKind;
use rustc_ast::{
    self as ast, GenericArg, GenericBound, GenericParamKind, ItemKind, MetaItem,
    TraitBoundModifiers, VariantData, WherePredicate,
};
use rustc_attr as attr;
use rustc_data_structures::flat_map_in_place::FlatMapInPlace;
use rustc_expand::base::{Annotatable, ExtCtxt};
use rustc_span::symbol::{sym, Ident};
use rustc_span::{Span, Symbol};
use smallvec::{smallvec, SmallVec};
use thin_vec::{thin_vec, ThinVec};

macro_rules! path {
    ($span:expr, $($part:ident)::*) => { vec![$(Ident::new(sym::$part, $span),)*] }
}

pub fn expand_deriving_smart_ptr(
    cx: &ExtCtxt<'_>,
    span: Span,
    _mitem: &MetaItem,
    item: &Annotatable,
    push: &mut dyn FnMut(Annotatable),
    _is_const: bool,
) {
    let (name_ident, generics) = if let Annotatable::Item(aitem) = item
        && let ItemKind::Struct(struct_data, g) = &aitem.kind
    {
        let is_transparent = aitem.attrs.iter().any(|attr| {
            attr::find_repr_attrs(cx.sess, attr)
                .into_iter()
                .any(|r| matches!(r, attr::ReprTransparent))
        });
        if !is_transparent {
            cx.dcx()
                .struct_span_err(
                    span,
                    "`SmartPointer` can only be derived on `struct`s with `#[repr(transparent)]`",
                )
                .emit();
            return;
        }
        if !matches!(
            struct_data,
            VariantData::Struct { fields, recovered: _ } | VariantData::Tuple(fields, _)
                if !fields.is_empty())
        {
            cx.dcx()
                .struct_span_err(
                    span,
                    "`SmartPointer` can only be derived on `struct`s with at least one field",
                )
                .emit();
            return;
        }
        (aitem.ident, g)
    } else {
        cx.dcx()
            .struct_span_err(
                span,
                "`SmartPointer` can only be derived on `struct`s with `#[repr(transparent)]`",
            )
            .emit();
        return;
    };

    // Convert generic parameters (from the struct) into generic args.
    let mut pointee_param = None;
    let mut multiple_pointee_diag: SmallVec<[_; 2]> = smallvec![];
    let self_params = generics
        .params
        .iter()
        .enumerate()
        .map(|(idx, p)| match p.kind {
            GenericParamKind::Lifetime => GenericArg::Lifetime(cx.lifetime(p.span(), p.ident)),
            GenericParamKind::Type { .. } => {
                if p.attrs().iter().any(|attr| attr.has_name(sym::pointee)) {
                    if pointee_param.is_some() {
                        multiple_pointee_diag.push(cx.dcx().struct_span_err(
                            p.span(),
                            "`SmartPointer` can only admit one type as pointee",
                        ));
                    } else {
                        pointee_param = Some(idx);
                    }
                }
                GenericArg::Type(cx.ty_ident(p.span(), p.ident))
            }
            GenericParamKind::Const { .. } => GenericArg::Const(cx.const_ident(p.span(), p.ident)),
        })
        .collect::<Vec<_>>();
    let Some(pointee_param_idx) = pointee_param else {
        cx.dcx().struct_span_err(
            span,
            "At least one generic type should be designated as `#[pointee]` in order to derive `SmartPointer` traits",
        ).emit();
        return;
    };
    if !multiple_pointee_diag.is_empty() {
        for diag in multiple_pointee_diag {
            diag.emit();
        }
        return;
    }

    // Create the type of `self`.
    let path = cx.path_all(span, false, vec![name_ident], self_params.clone());
    let self_type = cx.ty_path(path);

    // Declare helper function that adds implementation blocks.
    // FIXME(dingxiangfei2009): Investigate the set of attributes on target struct to be propagated to impls
    let attrs = thin_vec![cx.attr_word(sym::automatically_derived, span),];
    let mut add_impl_block = |generics, trait_symbol, trait_args| {
        let mut parts = path!(span, core::ops);
        parts.push(Ident::new(trait_symbol, span));
        let trait_path = cx.path_all(span, true, parts, trait_args);
        let trait_ref = cx.trait_ref(trait_path);
        let item = cx.item(
            span,
            Ident::empty(),
            attrs.clone(),
            ast::ItemKind::Impl(Box::new(ast::Impl {
                safety: ast::Safety::Default,
                polarity: ast::ImplPolarity::Positive,
                defaultness: ast::Defaultness::Final,
                constness: ast::Const::No,
                generics,
                of_trait: Some(trait_ref),
                self_ty: self_type.clone(),
                items: ThinVec::new(),
            })),
        );
        push(Annotatable::Item(item));
    };

    // Create unsized `self`, that is, one where the `#[pointee]` type arg is replaced with `__S`. For
    // example, instead of `MyType<'a, T>`, it will be `MyType<'a, __S>`.
    let s_ty = cx.ty_ident(span, Ident::new(sym::__S, span));
    let mut alt_self_params = self_params;
    alt_self_params[pointee_param_idx] = GenericArg::Type(s_ty.clone());
    let alt_self_type = cx.ty_path(cx.path_all(span, false, vec![name_ident], alt_self_params));

    // # Add `Unsize<__S>` bound to `#[pointee]` at the generic parameter location
    //
    // Find the `#[pointee]` parameter and add an `Unsize<__S>` bound to it.
    let mut impl_generics = generics.clone();
    let pointee_ty_ident = generics.params[pointee_param_idx].ident;
    let mut self_bounds;
    {
        let pointee = &mut impl_generics.params[pointee_param_idx];
        self_bounds = pointee.bounds.clone();
        if !contains_maybe_sized_bound(&self_bounds)
            && !contains_maybe_sized_bound_on_pointee(
                &generics.where_clause.predicates,
                pointee_ty_ident.name,
            )
        {
            cx.dcx()
                .struct_span_err(
                    pointee_ty_ident.span,
                    format!(
                        "`derive(SmartPointer)` requires {} to be marked `?Sized`",
                        pointee_ty_ident.name
                    ),
                )
                .emit();
            return;
        }
        let arg = GenericArg::Type(s_ty.clone());
        let unsize = cx.path_all(span, true, path!(span, core::marker::Unsize), vec![arg]);
        pointee.bounds.push(cx.trait_bound(unsize, false));
        // Drop `#[pointee]` attribute since it should not be recognized outside `derive(SmartPointer)`
        pointee.attrs.retain(|attr| !attr.has_name(sym::pointee));
    }

    // # Rewrite generic parameter bounds
    // For each bound `U: ..` in `struct<U: ..>`, make a new bound with `__S` in place of `#[pointee]`
    // Example:
    // ```
    // struct<
    //     U: Trait<T>,
    //     #[pointee] T: Trait<T> + ?Sized,
    //     V: Trait<T>> ...
    // ```
    // ... generates this `impl` generic parameters
    // ```
    // impl<
    //     U: Trait<T> + Trait<__S>,
    //     T: Trait<T> + ?Sized + Unsize<__S>, // (**)
    //     __S: Trait<__S> + ?Sized, // (*)
    //     V: Trait<T> + Trait<__S>> ...
    // ```
    // The new bound marked with (*) has to be done separately.
    // See next section
    for (idx, (params, orig_params)) in
        impl_generics.params.iter_mut().zip(&generics.params).enumerate()
    {
        // Default type parameters are rejected for `impl` block.
        // We should drop them now.
        match &mut params.kind {
            ast::GenericParamKind::Const { default, .. } => *default = None,
            ast::GenericParamKind::Type { default } => *default = None,
            ast::GenericParamKind::Lifetime => {}
        }
        // We CANNOT rewrite `#[pointee]` type parameter bounds.
        // This has been set in stone. (**)
        // So we skip over it.
        // Otherwise, we push extra bounds involving `__S`.
        if idx != pointee_param_idx {
            for bound in &orig_params.bounds {
                let mut bound = bound.clone();
                let mut substitution = TypeSubstitution {
                    from_name: pointee_ty_ident.name,
                    to_ty: &s_ty,
                    rewritten: false,
                };
                substitution.visit_param_bound(&mut bound, BoundKind::Bound);
                if substitution.rewritten {
                    // We found use of `#[pointee]` somewhere,
                    // so we make a new bound using `__S` in place of `#[pointee]`
                    params.bounds.push(bound);
                }
            }
        }
    }

    // # Insert `__S` type parameter
    //
    // We now insert `__S` with the missing bounds marked with (*) above.
    // We should also write the bounds from `#[pointee]` to `__S` as required by `Unsize<__S>`.
    {
        let mut substitution =
            TypeSubstitution { from_name: pointee_ty_ident.name, to_ty: &s_ty, rewritten: false };
        for bound in &mut self_bounds {
            substitution.visit_param_bound(bound, BoundKind::Bound);
        }
    }

    // # Rewrite `where` clauses
    //
    // Move on to `where` clauses.
    // Example:
    // ```
    // struct MyPointer<#[pointee] T, ..>
    // where
    //   U: Trait<V> + Trait<T>,
    //   Companion<T>: Trait<T>,
    //   T: Trait<T> + ?Sized,
    // { .. }
    // ```
    // ... will have a impl prelude like so
    // ```
    // impl<..> ..
    // where
    //   U: Trait<V> + Trait<T>,
    //   U: Trait<__S>,
    //   Companion<T>: Trait<T>,
    //   Companion<__S>: Trait<__S>,
    //   T: Trait<T> + ?Sized,
    //   __S: Trait<__S> + ?Sized,
    // ```
    //
    // We should also write a few new `where` bounds from `#[pointee] T` to `__S`
    // as well as any bound that indirectly involves the `#[pointee] T` type.
    for bound in &generics.where_clause.predicates {
        if let ast::WherePredicate::BoundPredicate(bound) = bound {
            let mut substitution = TypeSubstitution {
                from_name: pointee_ty_ident.name,
                to_ty: &s_ty,
                rewritten: false,
            };
            let mut predicate = ast::WherePredicate::BoundPredicate(ast::WhereBoundPredicate {
                span: bound.span,
                bound_generic_params: bound.bound_generic_params.clone(),
                bounded_ty: bound.bounded_ty.clone(),
                bounds: bound.bounds.clone(),
            });
            substitution.visit_where_predicate(&mut predicate);
            if substitution.rewritten {
                impl_generics.where_clause.predicates.push(predicate);
            }
        }
    }

    let extra_param = cx.typaram(span, Ident::new(sym::__S, span), self_bounds, None);
    impl_generics.params.insert(pointee_param_idx + 1, extra_param);

    // Add the impl blocks for `DispatchFromDyn` and `CoerceUnsized`.
    let gen_args = vec![GenericArg::Type(alt_self_type.clone())];
    add_impl_block(impl_generics.clone(), sym::DispatchFromDyn, gen_args.clone());
    add_impl_block(impl_generics.clone(), sym::CoerceUnsized, gen_args.clone());
}

fn contains_maybe_sized_bound_on_pointee(predicates: &[WherePredicate], pointee: Symbol) -> bool {
    for bound in predicates {
        if let ast::WherePredicate::BoundPredicate(bound) = bound
            && bound.bounded_ty.kind.is_simple_path().is_some_and(|name| name == pointee)
        {
            for bound in &bound.bounds {
                if is_maybe_sized_bound(bound) {
                    return true;
                }
            }
        }
    }
    false
}

fn is_maybe_sized_bound(bound: &GenericBound) -> bool {
    if let GenericBound::Trait(
        trait_ref,
        TraitBoundModifiers { polarity: ast::BoundPolarity::Maybe(_), .. },
    ) = bound
    {
        is_sized_marker(&trait_ref.trait_ref.path)
    } else {
        false
    }
}

fn contains_maybe_sized_bound(bounds: &[GenericBound]) -> bool {
    bounds.iter().any(is_maybe_sized_bound)
}

fn path_segment_is_exact_match(path_segments: &[ast::PathSegment], syms: &[Symbol]) -> bool {
    path_segments.iter().zip(syms).all(|(segment, &symbol)| segment.ident.name == symbol)
}

fn is_sized_marker(path: &ast::Path) -> bool {
    const CORE_UNSIZE: [Symbol; 3] = [sym::core, sym::marker, sym::Sized];
    const STD_UNSIZE: [Symbol; 3] = [sym::std, sym::marker, sym::Sized];
    if path.segments.len() == 4 && path.is_global() {
        path_segment_is_exact_match(&path.segments[1..], &CORE_UNSIZE)
            || path_segment_is_exact_match(&path.segments[1..], &STD_UNSIZE)
    } else if path.segments.len() == 3 {
        path_segment_is_exact_match(&path.segments, &CORE_UNSIZE)
            || path_segment_is_exact_match(&path.segments, &STD_UNSIZE)
    } else {
        *path == sym::Sized
    }
}

struct TypeSubstitution<'a> {
    from_name: Symbol,
    to_ty: &'a ast::Ty,
    rewritten: bool,
}

impl<'a> ast::mut_visit::MutVisitor for TypeSubstitution<'a> {
    fn visit_ty(&mut self, ty: &mut P<ast::Ty>) {
        if let Some(name) = ty.kind.is_simple_path()
            && name == self.from_name
        {
            **ty = self.to_ty.clone();
            self.rewritten = true;
        } else {
            ast::mut_visit::walk_ty(self, ty);
        }
    }

    fn visit_where_predicate(&mut self, where_predicate: &mut ast::WherePredicate) {
        match where_predicate {
            rustc_ast::WherePredicate::BoundPredicate(bound) => {
                bound
                    .bound_generic_params
                    .flat_map_in_place(|param| self.flat_map_generic_param(param));
                self.visit_ty(&mut bound.bounded_ty);
                for bound in &mut bound.bounds {
                    self.visit_param_bound(bound, BoundKind::Bound)
                }
            }
            rustc_ast::WherePredicate::RegionPredicate(_)
            | rustc_ast::WherePredicate::EqPredicate(_) => {}
        }
    }
}
