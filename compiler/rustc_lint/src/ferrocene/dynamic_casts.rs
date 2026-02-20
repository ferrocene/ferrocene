use std::ops::ControlFlow;

use rustc_abi::{FieldIdx, VariantIdx};
use rustc_hir::LangItem;
use rustc_hir::def_id::DefId;
use rustc_infer::traits::{
    ImplSource, ImplSourceUserDefinedData, Obligation, ObligationCause, ObligationCauseCode,
};
use rustc_middle::middle::codegen_fn_attrs::ferrocene::item_is_validated;
use rustc_middle::span_bug;
use rustc_middle::ty::adjustment::CustomCoerceUnsized;
use rustc_middle::ty::{
    self, ExistentialPredicate, GenericArgsRef, Instance, PolyTraitRef, Ty, TyCtxt,
    TypeSuperVisitable as _, TypeVisitable as _, TypingEnv,
};
use rustc_span::Span;
use rustc_trait_selection::traits::{ObligationCtxt, SelectionContext, supertraits};
use tracing::debug;

use crate::ferrocene::{InstantiateResult, LintState, UseKind};

impl<'tcx> LintState<'tcx> {
    pub(super) fn check_fn_ptr_coercion(
        &self,
        source: Ty<'tcx>,
        dst_trait: PolyTraitRef<'tcx>,
        try_instantiate: &mut impl FnMut(DefId, GenericArgsRef<'tcx>) -> InstantiateResult<'tcx>,
        span: Span,
    ) -> Option<UseKind<'tcx>> {
        debug!("check cast of {source:?} to function pointer");
        let tcx = self.tcx;

        match self.instance_of_ty(source, Some(dst_trait), try_instantiate, span) {
            Some(instance) => {
                if matches!(instance.def, ty::InstanceKind::Virtual(..)) {
                    // This is a `<dyn Trait>::method as fn()` cast.
                    // That shim is synthesized and therefore covered under the compiler
                    // qualification. It can't be called unless someone gets a `dyn Trait`, in which
                    // case we'll lint the unsizing cast.
                    None
                } else if item_is_validated(tcx, instance.def_id()).validated() {
                    None
                } else {
                    Some(UseKind::FnPtrCast(instance))
                }
            }
            // Uncaught fn pointer casts are ok because the post-mono pass will check them later.
            // FIXME: this is messy, split this out into `check_dyn_trait_coercion`
            None if Some(dst_trait.def_id()) == tcx.lang_items().fn_ptr_trait() => None,
            // FIXME: feature(unboxed_closures)
            None if source.is_adt() => None,
            // Don't remove this panic. If you do so before adding `check_dyn_trait_coercion` to
            // the post-mono pass, it will fail to catch real uses of unvalidated items in
            // non-degenerate programs.
            None => span_bug!(
                span,
                "unimplemented: pre-mono cast from {source} to dyn {dst_trait}() that fails to instantiate"
            ),
        }
    }

    /// Given a `source` expression that has an unsizing cast to `dest`, determine
    /// whether the cast is valid. If not, return a [`TraitObjectCast`](UseKind::TraitObjectCast) showing why not.
    ///
    /// This works in four main parts:
    /// 1. "Peel" as many types as possible. For example, if we are casting `Vec<Box<String>>` to
    ///    `Vec<Box<dyn Display + Clone + Sync>`, peel that to `String` and
    ///    `dyn Display + Clone + Sync`. We call these `coerce_src` and `coerce_dst`.
    /// 2. Determine all traits in `dest`'s type that have at least one method. For example,
    ///    `dyn Display + Clone + Sync` contains the traits `Display` and `Clone`.
    /// 3. For each trait, find `coerce_src`'s implementation of it. For example,
    ///    `impl Display for String`.
    /// 4. For each method in the impl, check whether it's validated. For example, we would check
    ///    `<String as Diplay>::fmt`, see that it's unvalidated, and return its `DefId` in the
    ///    `UseKind`.
    pub(super) fn check_dyn_trait_coercion(
        &self,
        dest_ty: Ty<'tcx>,
        source_ty: Ty<'tcx>,
        typing_env: TypingEnv<'tcx>,
        try_instantiate: &mut impl FnMut(DefId, GenericArgsRef<'tcx>) -> InstantiateResult<'tcx>,
        span: Span,
    ) -> Option<UseKind<'tcx>> {
        let tcx = self.tcx;

        let (coerce_src, coerce_dst) =
            self.peel_unsized_tys(source_ty, dest_ty, typing_env, span)?;
        debug!(
            "saw unsized coercion from {source_ty:?} -> {dest_ty:?} (peeled: {coerce_src:?} -> {coerce_dst:?})",
        );

        if matches!(coerce_src.kind(), ty::Dynamic(..) | ty::FnPtr(..)) {
            // upcasting from a `dyn Trait` to a `dyn SuperTrait`.
            // We already checked this when we originally cast to `dyn Trait`.
            return None;
        }

        let bound_traits = self.dyn_trait_refs(coerce_src, coerce_dst);
        // NOTE: this only checks functions directly on the `trait_ref`.
        // Supertraits are already handled in `dyn_trait_refs` as a separate trait.
        for trait_ref in bound_traits {
            // First, check if we are casting to a `dyn Fn*` trait.
            // If so, this is disallowed no matter what, for the same reason as casting
            // to a function pointer.
            if tcx.fn_trait_kind_from_def_id(trait_ref.def_id()).is_some() {
                if let Some(use_) =
                    self.check_fn_ptr_coercion(coerce_src, trait_ref, try_instantiate, span)
                {
                    return Some(use_);
                } else {
                    continue;
                }
            }

            if tcx
                .associated_item_def_ids(trait_ref.def_id())
                .iter()
                .find(|&id| tcx.def_kind(id).is_fn_like())
                .is_none()
            {
                // not possible to call any functions on this trait object, casting is always ok.
                continue;
            };
            let impl_ = self.find_trait_impl(trait_ref, typing_env, span);
            if let Some(unvalidated) = self.find_unvalidated_impl_fn(impl_, span) {
                return Some(UseKind::TraitObjectCast(unvalidated, coerce_src));
            }
        }
        None
    }

    /// Given a call to a function-like type, return the instantiated function definition,
    /// or `None` if we can't find it until it's been monomorphized.
    ///
    /// Panics if given a type that isn't callable.
    pub(super) fn instance_of_ty(
        &self,
        ty: Ty<'tcx>,
        fn_trait_ref: Option<PolyTraitRef<'tcx>>,
        try_instantiate: &mut impl FnMut(DefId, GenericArgsRef<'tcx>) -> InstantiateResult<'tcx>,
        span: Span,
    ) -> Option<Instance<'tcx>> {
        let tcx = self.tcx;

        match ty.kind() {
            ty::FnDef(maybe_trait_fn, generic_args) => {
                // Indeterminate results are handled later by a post-mono pass that checks the
                // instantiation is validated. For now just ignore errors.
                try_instantiate(*maybe_trait_fn, generic_args).instance()
            }
            ty::Closure(def_id, args) => {
                Some(Instance::resolve_closure(tcx, *def_id, args, ty::ClosureKind::FnOnce))
            }
            ty::CoroutineClosure(def_id, args) => {
                let coroutine_closure_def_id = *def_id;
                // See comment in `rustc_ty_utils::instance::resolve_associated_item`.
                let instance = if ty::ClosureKind::FnOnce == args.as_coroutine_closure().kind() {
                    Instance::new_raw(coroutine_closure_def_id, args)
                } else {
                    let trait_id = fn_trait_ref.unwrap().def_id();
                    let target_kind = tcx.fn_trait_kind_from_def_id(trait_id).unwrap();
                    Instance {
                        def: ty::InstanceKind::ConstructCoroutineInClosureShim {
                            coroutine_closure_def_id,
                            receiver_by_ref: target_kind != ty::ClosureKind::FnOnce,
                        },
                        args,
                    }
                };
                Some(instance)
            }
            // FIXME: `feature(unboxed_closures)`.
            // Right now we just ignore `fn_trait_ref`, but it's passed in here so that we can call
            // `find_trait_impl(fn_trait_ref.with_self_ty(ty)`.
            ty::Adt(..) => None,
            // Reference to a function or function pointer.
            ty::Ref(_, ty, _) => self.instance_of_ty(*ty, fn_trait_ref, try_instantiate, span),
            // We assume that all functions pointers are valid. Proof:
            // 1. If the function was validated, no problem.
            // 2. If the function was unvalidated, and is a literal or assigned to a local
            //    variable, then either:
            //    - We can resolve it to a concrete instance, in which case we would have caught it in `ZstLiteral` above.
            //    - We can't resolve it yet, but it stays a unique function type, so we will
            //    catch the call later in the post-mono pass.
            //    - We can't resolve it yet and it's cast to a function pointer so we don't
            //    have enough info to catch it post-mono when it's called. In this case we
            //    catch it in `ReifyFnPtr` above.
            // 3. If the function was passed as an argument, then either:
            //   - We were called by an unvalidated function. No problem.
            //   - We were called by a validated function. This lint will run on that
            //   function too, and we will catch it there at the time it is checked /
            //   monomorphized.
            // 4. If this is a closure then either:
            //   - It was defined in this function, in which case we treat it as also
            //   validated.
            //   - It was passed as an argument, which is ok by 3).
            //   - It is a global const/static, so we catch it in `NamedConst`/`StaticRef` above.
            ty::FnPtr(..) => None,
            other => span_bug!(span, "unsupported call kind {other:?}"),
        }
    }

    /// Given an `Unsize` coersion from `src_ty` to `dst_ty`, return the innermost "difference"
    /// between the two. Returns `None` if this isn't a cast to a trait object.
    ///
    /// This is quite similar to `struct_lockstep_tails_for_codegen`, except it considers all ZSTs,
    /// not just those at the tail.
    ///
    /// c.f. [Zulip](https://rust-lang.zulipchat.com/#narrow/channel/182449-t-compiler.2Fhelp/topic/Get.20a.20type's.20impl.20for.20a.20trait/with/570837962)
    ///
    /// See [`CoerceUnsized`](https://doc.rust-lang.org/std/ops/trait.CoerceUnsized.html) for a full
    /// list of types we need to handle here.
    ///
    /// This is adapted from `rustc_monomorphize::collector::find_tails_for_unsizing`.
    fn peel_unsized_tys(
        &self,
        src_ty: Ty<'tcx>,
        dst_ty: Ty<'tcx>,
        typing_env: TypingEnv<'tcx>,
        span: Span,
    ) -> Option<(Ty<'tcx>, Ty<'tcx>)> {
        debug!("unsize_ptr: {:?} => {:?}", src_ty, dst_ty);

        let tcx = self.tcx;
        let (mut src_inner, mut dst_inner) = (src_ty, dst_ty);

        let get_adt_field = |adt_def: ty::AdtDef<'_>, args, idx: FieldIdx| {
            let variant = adt_def.variant(VariantIdx::ZERO);
            let field_ty = variant.fields[idx].ty(tcx, args);
            tcx.normalize_erasing_regions(typing_env, field_ty)
        };

        loop {
            debug!("peel_tys step: src={src_inner:?}, dst={dst_inner:?}");
            match (src_inner.kind(), dst_inner.kind()) {
                /* End conditions. **/
                (_, ty::Dynamic(..)) => return Some((src_inner, dst_inner)),
                // There's only ever one unsizing coercion at once.
                // Even if this is a `[dyn Trait; N]`, we would have checked it earlier.
                // Return now so we don't crash trying to prove that the array implements `Trait`.
                (ty::Array(..), ty::Slice(..)) => return None,

                /* We've finished handling CoerceUnsized; now handle Unsize.
                 * From here onward, the only thing we'll ever hit in the loop is `Dynamic` or
                 * `Array` (handled above). */
                (ty::Ref(_, a, _), ty::Ref(_, b, _)) | (ty::RawPtr(a, _), ty::RawPtr(b, _)) => {
                    (src_inner, dst_inner) =
                        tcx.struct_lockstep_tails_for_codegen(*a, *b, typing_env);
                }

                /* Handle CoerceUnsized.
                 * This is the only part of the loop that recurses more than once. **/
                (ty::Pat(a, _), ty::Pat(b, _)) => {
                    (src_inner, dst_inner) = (*a, *b);
                }

                (ty::Adt(src_def, src_args), ty::Adt(dst_def, dst_args)) => {
                    assert_eq!(src_def.did(), dst_def.did());

                    if let Some(boxed) = src_inner.boxed_ty() {
                        src_inner = boxed;
                        dst_inner = dst_inner.boxed_ty().unwrap();
                        continue;
                    }

                    let field = match self.custom_coerce_unsize_info(src_inner, dst_inner, span) {
                        Some(CustomCoerceUnsized::Struct(idx)) => idx,
                        None => {
                            // Iterate this struct looking for a `!Sized` field.
                            let mut unsized_field = None;
                            for (idx, def) in
                                dst_def.variant(VariantIdx::ZERO).fields.iter_enumerated()
                            {
                                if !def.ty(tcx, dst_args).is_sized(tcx, typing_env) {
                                    unsized_field = Some(idx);
                                    break;
                                }
                            }
                            unsized_field.unwrap_or_else(|| {
                                span_bug!(span, "Adt with no CoerceUnsized impl and no !Sized field? {dst_inner:?}");
                            })
                        }
                    };
                    src_inner = get_adt_field(*src_def, src_args, field);
                    dst_inner = get_adt_field(*dst_def, dst_args, field);
                }
                _ => span_bug!(
                    span,
                    "mismatched types trying to coerce from {src_inner:?} to {dst_inner:?}"
                ),
            }
        }
    }

    // copied from rustc_monomorphize
    fn custom_coerce_unsize_info(
        &self,
        source_ty: Ty<'tcx>,
        target_ty: Ty<'tcx>,
        span: Span,
    ) -> Option<CustomCoerceUnsized> {
        let tcx = self.tcx;
        let trait_ref = ty::TraitRef::new(
            tcx,
            tcx.require_lang_item(LangItem::CoerceUnsized, span),
            [source_ty, target_ty],
        );

        match tcx.codegen_select_candidate(
            ty::TypingEnv::fully_monomorphized().as_query_input(trait_ref),
        ) {
            Ok(ImplSource::UserDefined(ImplSourceUserDefinedData { impl_def_id, .. })) => {
                Some(tcx.coerce_unsized_info(impl_def_id).unwrap().custom_kind.unwrap())
            }
            _ => None,
        }
    }

    /// Given an `impl`, find the first associated function that isn't validated.
    ///
    /// FIXME: list all unvalidated functions, not just the first.
    fn find_unvalidated_impl_fn(
        &self,
        impl_source: ImplSource<'tcx, ()>,
        span: Span,
    ) -> Option<DefId> {
        let tcx = self.tcx;

        let impl_block = match impl_source {
            ImplSource::UserDefined(ImplSourceUserDefinedData {
                impl_def_id,
                args: _,
                nested: _,
            }) => impl_def_id,
            // builtin impls are always ok
            ImplSource::Builtin(..) => return None,
            param @ ImplSource::Param(_) => {
                span_bug!(span, "don't know how to handle nested obligations in {param:?}")
            }
        };

        let trait_to_impl_map = tcx.impl_item_implementor_ids(impl_block);
        for trait_item in tcx.associated_item_def_ids(tcx.impl_trait_id(impl_block)) {
            debug!("considering {trait_item:?}");
            if !tcx.def_kind(trait_item).is_fn_like() {
                debug!("ignoring non-fn {trait_item:?}");
                continue;
            }

            // Need this map so we consider default trait fns even if they're not mentioned in the
            // impl block.
            let impl_fn = *trait_to_impl_map.get(trait_item).unwrap_or(trait_item);

            if item_is_validated(tcx, impl_fn).validated() {
                continue;
            }

            debug!("found unvalidated method {impl_fn:?}");
            // This function in the impl needs to be marked with `prevalidated`.
            return Some(impl_fn);
        }
        return None;
    }

    fn find_trait_impl(
        &self,
        trait_ref: PolyTraitRef<'tcx>,
        typing_env: TypingEnv<'tcx>,
        span: Span,
    ) -> ImplSource<'tcx, ()> {
        match self.try_find_trait_impl(trait_ref, typing_env, span) {
            Some(found) => found,
            None => span_bug!(span, "failed to resolve impl: {trait_ref:?}"),
        }
    }

    /// Given a `Trait<Ty>` reference, find `impl Trait for Ty`.
    ///
    /// This calls into the [trait solver] to select a suitable impl block.
    ///
    /// c.f. [`hax::exporter::traits::resolution::shallow_resolve_trait_ref`](https://github.com/AeneasVerif/hax/blob/3c2b6f01af4a4362dd855b811aa910ad173d546f/frontend/exporter/src/traits/resolution.rs#L666)
    ///
    /// Ideally, this would never return None, but sometimes our code is buggy ... failures here
    /// only degrade the diagnostic, they don't cause soundness issues.
    ///
    /// [trait solver]: https://rustc-dev-guide.rust-lang.org/traits/resolution.html
    fn try_find_trait_impl(
        &self,
        trait_ref: PolyTraitRef<'tcx>,
        typing_env: TypingEnv<'tcx>,
        span: Span,
    ) -> Option<ImplSource<'tcx, ()>> {
        use rustc_infer::infer::TyCtxtInferExt;

        let tcx = self.tcx;
        let (infcx, param_env) =
            tcx.infer_ctxt().ignoring_regions().build_with_typing_env(typing_env);
        debug!("resolving impl for {trait_ref:?}");

        // Find the impl block.
        let mut selcx = SelectionContext::new(&infcx);
        let cause = ObligationCause::new(span, self.item, ObligationCauseCode::ExprAssignable);
        // Normalize the trait ref.
        let trait_ref = tcx.normalize_erasing_regions(typing_env, trait_ref);
        // method selection doesn't care about regions.
        let trait_ref = tcx.instantiate_bound_regions_with_erased(trait_ref);
        let obligation = Obligation::new(tcx, cause, param_env, trait_ref);
        let selection = match selcx.select(&obligation) {
            Ok(selection) => selection?,
            Err(e) => {
                debug!("type checking failed for trait upcast? {e:?}");
                return None;
            }
        };
        debug!("selected {selection:?}");

        // Sanity check: make sure all `where` clauses on our `impl` are upheld.
        let ocx = ObligationCtxt::new(&infcx);
        let impl_source = selection.map(|o| {
            debug!("registering obligation {o:?}");
            ocx.register_obligation(o)
        });
        let errors = ocx.evaluate_obligations_error_on_ambiguity();
        if !errors.is_empty() {
            debug!("impl obligations not met for trait upcast: {errors:?}");
            return None;
        }

        let normalized_impl =
            tcx.erase_and_anonymize_regions(infcx.resolve_vars_if_possible(impl_source));
        debug!("found impl {normalized_impl:?}");
        Some(normalized_impl)
    }

    /// Given a Rust type, find (at any level of nesting) `dyn Trait` objects contained within it that
    /// have at least one method.
    /// For example, `dyn Send + Sync + Display + Clone` would return `[Display, Clone]`.
    fn dyn_trait_refs(
        &self,
        source_ty: Ty<'tcx>,
        dst_ty: Ty<'tcx>,
    ) -> Vec<ty::Binder<'tcx, ty::TraitRef<'tcx>>> {
        struct FindDynTraitVisitor<'tcx>(TyCtxt<'tcx>, Ty<'tcx>);

        impl<'tcx> ty::TypeVisitor<TyCtxt<'tcx>> for FindDynTraitVisitor<'tcx> {
            type Result = ControlFlow<Vec<ty::Binder<'tcx, ty::TraitRef<'tcx>>>>;

            fn visit_ty(&mut self, t: Ty<'tcx>) -> Self::Result {
                match t.kind() {
                    ty::Dynamic(bound_predicates, _lifetime) => {
                        let mut traits = vec![];
                        for predicate in *bound_predicates {
                            debug!("considering {predicate:?}");
                            let trait_ = predicate
                                .map_bound(|p| match p {
                                    // auto traits do not allow calling methods
                                    ExistentialPredicate::AutoTrait(_) => None,
                                    // We don't care about associated type bounds.
                                    // They restrict which impl is selected, but that's all they do.
                                    // We already require the implementation of the trait to be validated
                                    // (that's the `Trait` predicate below), so which impl gets picked
                                    // doesn't matter as long we know which one it is.
                                    ExistentialPredicate::Projection(_) => None,
                                    ExistentialPredicate::Trait(t) => Some(t),
                                })
                                .transpose();
                            if let Some(t) = trait_ {
                                let t = t.with_self_ty(self.0, self.1);
                                traits.push(t);
                                traits.extend(supertraits(self.0, t));
                            }
                        }
                        // FIXME: is it possible to have multiple Dynamic types in a single top-level
                        // type? how? maybe with an enum?
                        ControlFlow::Break(traits)
                    }
                    _ => t.super_visit_with(self),
                }
            }
        }

        let cf = dst_ty.visit_with(&mut FindDynTraitVisitor(self.tcx, source_ty));
        cf.break_value().unwrap_or_default()
    }
}
