//! Check properties that are required by built-in traits and set
//! up data structures required by type-checking/codegen.

use crate::errors;

use rustc_data_structures::fx::FxHashSet;
use rustc_errors::{ErrorGuaranteed, MultiSpan};
use rustc_hir as hir;
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_hir::lang_items::LangItem;
use rustc_hir::ItemKind;
use rustc_infer::infer::outlives::env::OutlivesEnvironment;
use rustc_infer::infer::{self, RegionResolutionError};
use rustc_infer::infer::{DefineOpaqueTypes, TyCtxtInferExt};
use rustc_infer::traits::Obligation;
use rustc_middle::ty::adjustment::CoerceUnsizedInfo;
use rustc_middle::ty::{self, suggest_constraining_type_params, Ty, TyCtxt, TypeVisitableExt};
use rustc_span::Span;
use rustc_trait_selection::traits::error_reporting::TypeErrCtxtExt;
use rustc_trait_selection::traits::misc::{
    type_allowed_to_implement_const_param_ty, type_allowed_to_implement_copy,
    ConstParamTyImplementationError, CopyImplementationError, InfringingFieldsReason,
};
use rustc_trait_selection::traits::ObligationCtxt;
use rustc_trait_selection::traits::{self, ObligationCause};
use std::collections::BTreeMap;

pub fn check_trait(tcx: TyCtxt<'_>, trait_def_id: DefId) {
    let lang_items = tcx.lang_items();
    Checker { tcx, trait_def_id }
        .check(lang_items.drop_trait(), visit_implementation_of_drop)
        .check(lang_items.copy_trait(), visit_implementation_of_copy)
        .check(lang_items.const_param_ty_trait(), visit_implementation_of_const_param_ty)
        .check(lang_items.coerce_unsized_trait(), visit_implementation_of_coerce_unsized)
        .check(lang_items.dispatch_from_dyn_trait(), visit_implementation_of_dispatch_from_dyn);
}

struct Checker<'tcx> {
    tcx: TyCtxt<'tcx>,
    trait_def_id: DefId,
}

impl<'tcx> Checker<'tcx> {
    fn check<F>(&self, trait_def_id: Option<DefId>, mut f: F) -> &Self
    where
        F: FnMut(TyCtxt<'tcx>, LocalDefId),
    {
        if Some(self.trait_def_id) == trait_def_id {
            for &impl_def_id in self.tcx.hir().trait_impls(self.trait_def_id) {
                f(self.tcx, impl_def_id);
            }
        }
        self
    }
}

fn visit_implementation_of_drop(tcx: TyCtxt<'_>, impl_did: LocalDefId) {
    // Destructors only work on local ADT types.
    match tcx.type_of(impl_did).instantiate_identity().kind() {
        ty::Adt(def, _) if def.did().is_local() => return,
        ty::Error(_) => return,
        _ => {}
    }

    let impl_ = tcx.hir().expect_item(impl_did).expect_impl();

    tcx.sess.emit_err(errors::DropImplOnWrongItem { span: impl_.self_ty.span });
}

fn visit_implementation_of_copy(tcx: TyCtxt<'_>, impl_did: LocalDefId) {
    debug!("visit_implementation_of_copy: impl_did={:?}", impl_did);

    let self_type = tcx.type_of(impl_did).instantiate_identity();
    debug!("visit_implementation_of_copy: self_type={:?} (bound)", self_type);

    let param_env = tcx.param_env(impl_did);
    assert!(!self_type.has_escaping_bound_vars());

    debug!("visit_implementation_of_copy: self_type={:?} (free)", self_type);

    let span = match tcx.hir().expect_item(impl_did).expect_impl() {
        hir::Impl { polarity: hir::ImplPolarity::Negative(_), .. } => return,
        hir::Impl { self_ty, .. } => self_ty.span,
    };

    let cause = traits::ObligationCause::misc(span, impl_did);
    match type_allowed_to_implement_copy(tcx, param_env, self_type, cause) {
        Ok(()) => {}
        Err(CopyImplementationError::InfringingFields(fields)) => {
            infringing_fields_error(tcx, fields, LangItem::Copy, impl_did, span);
        }
        Err(CopyImplementationError::NotAnAdt) => {
            tcx.sess.emit_err(errors::CopyImplOnNonAdt { span });
        }
        Err(CopyImplementationError::HasDestructor) => {
            tcx.sess.emit_err(errors::CopyImplOnTypeWithDtor { span });
        }
    }
}

fn visit_implementation_of_const_param_ty(tcx: TyCtxt<'_>, impl_did: LocalDefId) {
    let self_type = tcx.type_of(impl_did).instantiate_identity();
    assert!(!self_type.has_escaping_bound_vars());

    let param_env = tcx.param_env(impl_did);

    let span = match tcx.hir().expect_item(impl_did).expect_impl() {
        hir::Impl { polarity: hir::ImplPolarity::Negative(_), .. } => return,
        impl_ => impl_.self_ty.span,
    };

    let cause = traits::ObligationCause::misc(span, impl_did);
    match type_allowed_to_implement_const_param_ty(tcx, param_env, self_type, cause) {
        Ok(()) => {}
        Err(ConstParamTyImplementationError::InfrigingFields(fields)) => {
            infringing_fields_error(tcx, fields, LangItem::ConstParamTy, impl_did, span);
        }
        Err(ConstParamTyImplementationError::NotAnAdtOrBuiltinAllowed) => {
            tcx.sess.emit_err(errors::ConstParamTyImplOnNonAdt { span });
        }
    }
}

fn visit_implementation_of_coerce_unsized(tcx: TyCtxt<'_>, impl_did: LocalDefId) {
    debug!("visit_implementation_of_coerce_unsized: impl_did={:?}", impl_did);

    // Just compute this for the side-effects, in particular reporting
    // errors; other parts of the code may demand it for the info of
    // course.
    let span = tcx.def_span(impl_did);
    tcx.at(span).coerce_unsized_info(impl_did);
}

fn visit_implementation_of_dispatch_from_dyn(tcx: TyCtxt<'_>, impl_did: LocalDefId) {
    debug!("visit_implementation_of_dispatch_from_dyn: impl_did={:?}", impl_did);

    let span = tcx.def_span(impl_did);

    let dispatch_from_dyn_trait = tcx.require_lang_item(LangItem::DispatchFromDyn, Some(span));

    let source = tcx.type_of(impl_did).instantiate_identity();
    assert!(!source.has_escaping_bound_vars());
    let target = {
        let trait_ref = tcx.impl_trait_ref(impl_did).unwrap().instantiate_identity();
        assert_eq!(trait_ref.def_id, dispatch_from_dyn_trait);

        trait_ref.args.type_at(1)
    };

    debug!("visit_implementation_of_dispatch_from_dyn: {:?} -> {:?}", source, target);

    let param_env = tcx.param_env(impl_did);

    let infcx = tcx.infer_ctxt().build();
    let cause = ObligationCause::misc(span, impl_did);

    // Later parts of the compiler rely on all DispatchFromDyn types to be ABI-compatible with raw
    // pointers. This is enforced here: we only allow impls for references, raw pointers, and things
    // that are effectively repr(transparent) newtypes around types that already hav a
    // DispatchedFromDyn impl. We cannot literally use repr(transparent) on those tpyes since some
    // of them support an allocator, but we ensure that for the cases where the type implements this
    // trait, they *do* satisfy the repr(transparent) rules, and then we assume that everything else
    // in the compiler (in particular, all the call ABI logic) will treat them as repr(transparent)
    // even if they do not carry that attribute.
    use rustc_type_ir::sty::TyKind::*;
    match (source.kind(), target.kind()) {
        (&Ref(r_a, _, mutbl_a), Ref(r_b, _, mutbl_b))
            if infcx.at(&cause, param_env).eq(DefineOpaqueTypes::No, r_a, *r_b).is_ok()
                && mutbl_a == *mutbl_b => {}
        (&RawPtr(tm_a), &RawPtr(tm_b)) if tm_a.mutbl == tm_b.mutbl => (),
        (&Adt(def_a, args_a), &Adt(def_b, args_b)) if def_a.is_struct() && def_b.is_struct() => {
            if def_a != def_b {
                let source_path = tcx.def_path_str(def_a.did());
                let target_path = tcx.def_path_str(def_b.did());

                tcx.sess.emit_err(errors::DispatchFromDynCoercion {
                    span,
                    trait_name: "DispatchFromDyn",
                    note: true,
                    source_path,
                    target_path,
                });

                return;
            }

            if def_a.repr().c() || def_a.repr().packed() {
                tcx.sess.emit_err(errors::DispatchFromDynRepr { span });
            }

            let fields = &def_a.non_enum_variant().fields;

            let coerced_fields = fields
                .iter()
                .filter(|field| {
                    let ty_a = field.ty(tcx, args_a);
                    let ty_b = field.ty(tcx, args_b);

                    if let Ok(layout) = tcx.layout_of(param_env.and(ty_a)) {
                        if layout.is_1zst() {
                            // ignore 1-ZST fields
                            return false;
                        }
                    }

                    if let Ok(ok) =
                        infcx.at(&cause, param_env).eq(DefineOpaqueTypes::No, ty_a, ty_b)
                    {
                        if ok.obligations.is_empty() {
                            tcx.sess.emit_err(errors::DispatchFromDynZST {
                                span,
                                name: field.name,
                                ty: ty_a,
                            });

                            return false;
                        }
                    }

                    return true;
                })
                .collect::<Vec<_>>();

            if coerced_fields.is_empty() {
                tcx.sess.emit_err(errors::DispatchFromDynSingle {
                    span,
                    trait_name: "DispatchFromDyn",
                    note: true,
                });
            } else if coerced_fields.len() > 1 {
                tcx.sess.emit_err(errors::DispatchFromDynMulti {
                    span,
                    coercions_note: true,
                    number: coerced_fields.len(),
                    coercions: coerced_fields
                        .iter()
                        .map(|field| {
                            format!(
                                "`{}` (`{}` to `{}`)",
                                field.name,
                                field.ty(tcx, args_a),
                                field.ty(tcx, args_b),
                            )
                        })
                        .collect::<Vec<_>>()
                        .join(", "),
                });
            } else {
                let ocx = ObligationCtxt::new(&infcx);
                for field in coerced_fields {
                    ocx.register_obligation(Obligation::new(
                        tcx,
                        cause.clone(),
                        param_env,
                        ty::TraitRef::new(
                            tcx,
                            dispatch_from_dyn_trait,
                            [field.ty(tcx, args_a), field.ty(tcx, args_b)],
                        ),
                    ));
                }
                let errors = ocx.select_all_or_error();
                if !errors.is_empty() {
                    infcx.err_ctxt().report_fulfillment_errors(errors);
                }

                // Finally, resolve all regions.
                let outlives_env = OutlivesEnvironment::new(param_env);
                let _ = ocx.resolve_regions_and_report_errors(impl_did, &outlives_env);
            }
        }
        _ => {
            tcx.sess.emit_err(errors::CoerceUnsizedMay { span, trait_name: "DispatchFromDyn" });
        }
    }
}

pub fn coerce_unsized_info<'tcx>(tcx: TyCtxt<'tcx>, impl_did: LocalDefId) -> CoerceUnsizedInfo {
    debug!("compute_coerce_unsized_info(impl_did={:?})", impl_did);
    let span = tcx.def_span(impl_did);

    let coerce_unsized_trait = tcx.require_lang_item(LangItem::CoerceUnsized, Some(span));

    let unsize_trait = tcx.require_lang_item(LangItem::Unsize, Some(span));

    let source = tcx.type_of(impl_did).instantiate_identity();
    let trait_ref = tcx.impl_trait_ref(impl_did).unwrap().instantiate_identity();
    assert_eq!(trait_ref.def_id, coerce_unsized_trait);
    let target = trait_ref.args.type_at(1);
    debug!("visit_implementation_of_coerce_unsized: {:?} -> {:?} (bound)", source, target);

    let param_env = tcx.param_env(impl_did);
    assert!(!source.has_escaping_bound_vars());

    let err_info = CoerceUnsizedInfo { custom_kind: None };

    debug!("visit_implementation_of_coerce_unsized: {:?} -> {:?} (free)", source, target);

    let infcx = tcx.infer_ctxt().build();
    let cause = ObligationCause::misc(span, impl_did);
    let check_mutbl = |mt_a: ty::TypeAndMut<'tcx>,
                       mt_b: ty::TypeAndMut<'tcx>,
                       mk_ptr: &dyn Fn(Ty<'tcx>) -> Ty<'tcx>| {
        if mt_a.mutbl < mt_b.mutbl {
            infcx
                .err_ctxt()
                .report_mismatched_types(
                    &cause,
                    mk_ptr(mt_b.ty),
                    target,
                    ty::error::TypeError::Mutability,
                )
                .emit();
        }
        (mt_a.ty, mt_b.ty, unsize_trait, None)
    };
    let (source, target, trait_def_id, kind) = match (source.kind(), target.kind()) {
        (&ty::Ref(r_a, ty_a, mutbl_a), &ty::Ref(r_b, ty_b, mutbl_b)) => {
            infcx.sub_regions(infer::RelateObjectBound(span), r_b, r_a);
            let mt_a = ty::TypeAndMut { ty: ty_a, mutbl: mutbl_a };
            let mt_b = ty::TypeAndMut { ty: ty_b, mutbl: mutbl_b };
            check_mutbl(mt_a, mt_b, &|ty| Ty::new_imm_ref(tcx, r_b, ty))
        }

        (&ty::Ref(_, ty_a, mutbl_a), &ty::RawPtr(mt_b)) => {
            let mt_a = ty::TypeAndMut { ty: ty_a, mutbl: mutbl_a };
            check_mutbl(mt_a, mt_b, &|ty| Ty::new_imm_ptr(tcx, ty))
        }

        (&ty::RawPtr(mt_a), &ty::RawPtr(mt_b)) => {
            check_mutbl(mt_a, mt_b, &|ty| Ty::new_imm_ptr(tcx, ty))
        }

        (&ty::Adt(def_a, args_a), &ty::Adt(def_b, args_b))
            if def_a.is_struct() && def_b.is_struct() =>
        {
            if def_a != def_b {
                let source_path = tcx.def_path_str(def_a.did());
                let target_path = tcx.def_path_str(def_b.did());
                tcx.sess.emit_err(errors::DispatchFromDynSame {
                    span,
                    trait_name: "CoerceUnsized",
                    note: true,
                    source_path,
                    target_path,
                });
                return err_info;
            }

            // Here we are considering a case of converting
            // `S<P0...Pn>` to `S<Q0...Qn>`. As an example, let's imagine a struct `Foo<T, U>`,
            // which acts like a pointer to `U`, but carries along some extra data of type `T`:
            //
            //     struct Foo<T, U> {
            //         extra: T,
            //         ptr: *mut U,
            //     }
            //
            // We might have an impl that allows (e.g.) `Foo<T, [i32; 3]>` to be unsized
            // to `Foo<T, [i32]>`. That impl would look like:
            //
            //   impl<T, U: Unsize<V>, V> CoerceUnsized<Foo<T, V>> for Foo<T, U> {}
            //
            // Here `U = [i32; 3]` and `V = [i32]`. At runtime,
            // when this coercion occurs, we would be changing the
            // field `ptr` from a thin pointer of type `*mut [i32;
            // 3]` to a fat pointer of type `*mut [i32]` (with
            // extra data `3`). **The purpose of this check is to
            // make sure that we know how to do this conversion.**
            //
            // To check if this impl is legal, we would walk down
            // the fields of `Foo` and consider their types with
            // both substitutes. We are looking to find that
            // exactly one (non-phantom) field has changed its
            // type, which we will expect to be the pointer that
            // is becoming fat (we could probably generalize this
            // to multiple thin pointers of the same type becoming
            // fat, but we don't). In this case:
            //
            // - `extra` has type `T` before and type `T` after
            // - `ptr` has type `*mut U` before and type `*mut V` after
            //
            // Since just one field changed, we would then check
            // that `*mut U: CoerceUnsized<*mut V>` is implemented
            // (in other words, that we know how to do this
            // conversion). This will work out because `U:
            // Unsize<V>`, and we have a builtin rule that `*mut
            // U` can be coerced to `*mut V` if `U: Unsize<V>`.
            let fields = &def_a.non_enum_variant().fields;
            let diff_fields = fields
                .iter_enumerated()
                .filter_map(|(i, f)| {
                    let (a, b) = (f.ty(tcx, args_a), f.ty(tcx, args_b));

                    if tcx.type_of(f.did).instantiate_identity().is_phantom_data() {
                        // Ignore PhantomData fields
                        return None;
                    }

                    // Ignore fields that aren't changed; it may
                    // be that we could get away with subtyping or
                    // something more accepting, but we use
                    // equality because we want to be able to
                    // perform this check without computing
                    // variance where possible. (This is because
                    // we may have to evaluate constraint
                    // expressions in the course of execution.)
                    // See e.g., #41936.
                    if let Ok(ok) = infcx.at(&cause, param_env).eq(DefineOpaqueTypes::No, a, b) {
                        if ok.obligations.is_empty() {
                            return None;
                        }
                    }

                    // Collect up all fields that were significantly changed
                    // i.e., those that contain T in coerce_unsized T -> U
                    Some((i, a, b))
                })
                .collect::<Vec<_>>();

            if diff_fields.is_empty() {
                tcx.sess.emit_err(errors::CoerceUnsizedOneField {
                    span,
                    trait_name: "CoerceUnsized",
                    note: true,
                });
                return err_info;
            } else if diff_fields.len() > 1 {
                let item = tcx.hir().expect_item(impl_did);
                let span = if let ItemKind::Impl(hir::Impl { of_trait: Some(t), .. }) = &item.kind {
                    t.path.span
                } else {
                    tcx.def_span(impl_did)
                };

                tcx.sess.emit_err(errors::CoerceUnsizedMulti {
                    span,
                    coercions_note: true,
                    number: diff_fields.len(),
                    coercions: diff_fields
                        .iter()
                        .map(|&(i, a, b)| format!("`{}` (`{}` to `{}`)", fields[i].name, a, b))
                        .collect::<Vec<_>>()
                        .join(", "),
                });

                return err_info;
            }

            let (i, a, b) = diff_fields[0];
            let kind = ty::adjustment::CustomCoerceUnsized::Struct(i);
            (a, b, coerce_unsized_trait, Some(kind))
        }

        _ => {
            tcx.sess.emit_err(errors::DispatchFromDynStruct { span, trait_name: "CoerceUnsized" });
            return err_info;
        }
    };

    // Register an obligation for `A: Trait<B>`.
    let ocx = ObligationCtxt::new(&infcx);
    let cause = traits::ObligationCause::misc(span, impl_did);
    let obligation = Obligation::new(
        tcx,
        cause,
        param_env,
        ty::TraitRef::new(tcx, trait_def_id, [source, target]),
    );
    ocx.register_obligation(obligation);
    let errors = ocx.select_all_or_error();
    if !errors.is_empty() {
        infcx.err_ctxt().report_fulfillment_errors(errors);
    }

    // Finally, resolve all regions.
    let outlives_env = OutlivesEnvironment::new(param_env);
    let _ = ocx.resolve_regions_and_report_errors(impl_did, &outlives_env);

    CoerceUnsizedInfo { custom_kind: kind }
}

fn infringing_fields_error(
    tcx: TyCtxt<'_>,
    fields: Vec<(&ty::FieldDef, Ty<'_>, InfringingFieldsReason<'_>)>,
    lang_item: LangItem,
    impl_did: LocalDefId,
    impl_span: Span,
) -> ErrorGuaranteed {
    let trait_did = tcx.require_lang_item(lang_item, Some(impl_span));

    let trait_name = tcx.def_path_str(trait_did);

    // We'll try to suggest constraining type parameters to fulfill the requirements of
    // their `Copy` implementation.
    let mut errors: BTreeMap<_, Vec<_>> = Default::default();
    let mut bounds = vec![];

    let mut seen_tys = FxHashSet::default();

    let mut label_spans = Vec::new();

    for (field, ty, reason) in fields {
        // Only report an error once per type.
        if !seen_tys.insert(ty) {
            continue;
        }

        label_spans.push(tcx.def_span(field.did));

        match reason {
            InfringingFieldsReason::Fulfill(fulfillment_errors) => {
                for error in fulfillment_errors {
                    let error_predicate = error.obligation.predicate;
                    // Only note if it's not the root obligation, otherwise it's trivial and
                    // should be self-explanatory (i.e. a field literally doesn't implement Copy).

                    // FIXME: This error could be more descriptive, especially if the error_predicate
                    // contains a foreign type or if it's a deeply nested type...
                    if error_predicate != error.root_obligation.predicate {
                        errors
                            .entry((ty.to_string(), error_predicate.to_string()))
                            .or_default()
                            .push(error.obligation.cause.span);
                    }
                    if let ty::PredicateKind::Clause(ty::ClauseKind::Trait(ty::TraitPredicate {
                        trait_ref,
                        polarity: ty::ImplPolarity::Positive,
                        ..
                    })) = error_predicate.kind().skip_binder()
                    {
                        let ty = trait_ref.self_ty();
                        if let ty::Param(_) = ty.kind() {
                            bounds.push((
                                format!("{ty}"),
                                trait_ref.print_only_trait_path().to_string(),
                                Some(trait_ref.def_id),
                            ));
                        }
                    }
                }
            }
            InfringingFieldsReason::Regions(region_errors) => {
                for error in region_errors {
                    let ty = ty.to_string();
                    match error {
                        RegionResolutionError::ConcreteFailure(origin, a, b) => {
                            let predicate = format!("{b}: {a}");
                            errors
                                .entry((ty.clone(), predicate.clone()))
                                .or_default()
                                .push(origin.span());
                            if let ty::RegionKind::ReEarlyBound(ebr) = *b && ebr.has_name() {
                                        bounds.push((b.to_string(), a.to_string(), None));
                                    }
                        }
                        RegionResolutionError::GenericBoundFailure(origin, a, b) => {
                            let predicate = format!("{a}: {b}");
                            errors
                                .entry((ty.clone(), predicate.clone()))
                                .or_default()
                                .push(origin.span());
                            if let infer::region_constraints::GenericKind::Param(_) = a {
                                bounds.push((a.to_string(), b.to_string(), None));
                            }
                        }
                        _ => continue,
                    }
                }
            }
        }
    }
    let mut notes = Vec::new();
    for ((ty, error_predicate), spans) in errors {
        let span: MultiSpan = spans.into();
        notes.push(errors::ImplForTyRequires {
            span,
            error_predicate,
            trait_name: trait_name.clone(),
            ty,
        });
    }

    let mut err = tcx.sess.create_err(errors::TraitCannotImplForTy {
        span: impl_span,
        trait_name,
        label_spans,
        notes,
    });

    suggest_constraining_type_params(
        tcx,
        tcx.hir().get_generics(impl_did).expect("impls always have generics"),
        &mut err,
        bounds
            .iter()
            .map(|(param, constraint, def_id)| (param.as_str(), constraint.as_str(), *def_id)),
        None,
    );

    err.emit()
}
