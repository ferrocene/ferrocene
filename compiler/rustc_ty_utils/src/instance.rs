use rustc_errors::ErrorGuaranteed;
use rustc_hir::def_id::DefId;
use rustc_infer::infer::TyCtxtInferExt;
use rustc_middle::query::Providers;
use rustc_middle::traits::{BuiltinImplSource, CodegenObligationError};
use rustc_middle::ty::GenericArgsRef;
use rustc_middle::ty::{self, Instance, TyCtxt, TypeVisitableExt};
use rustc_span::sym;
use rustc_trait_selection::traits;
use rustc_type_ir::ClosureKind;
use traits::{translate_args, Reveal};

use crate::errors::UnexpectedFnPtrAssociatedItem;

fn resolve_instance<'tcx>(
    tcx: TyCtxt<'tcx>,
    key: ty::ParamEnvAnd<'tcx, (DefId, GenericArgsRef<'tcx>)>,
) -> Result<Option<Instance<'tcx>>, ErrorGuaranteed> {
    let (param_env, (def_id, args)) = key.into_parts();

    let result = if let Some(trait_def_id) = tcx.trait_of_item(def_id) {
        debug!(" => associated item, attempting to find impl in param_env {:#?}", param_env);
        resolve_associated_item(
            tcx,
            def_id,
            param_env,
            trait_def_id,
            tcx.normalize_erasing_regions(param_env, args),
        )
    } else {
        let def = if tcx.intrinsic(def_id).is_some() {
            debug!(" => intrinsic");
            ty::InstanceDef::Intrinsic(def_id)
        } else if Some(def_id) == tcx.lang_items().drop_in_place_fn() {
            let ty = args.type_at(0);

            if ty.needs_drop(tcx, param_env) {
                debug!(" => nontrivial drop glue");
                match *ty.kind() {
                    ty::Closure(..)
                    | ty::CoroutineClosure(..)
                    | ty::Coroutine(..)
                    | ty::Tuple(..)
                    | ty::Adt(..)
                    | ty::Dynamic(..)
                    | ty::Array(..)
                    | ty::Slice(..) => {}
                    // Drop shims can only be built from ADTs.
                    _ => return Ok(None),
                }

                ty::InstanceDef::DropGlue(def_id, Some(ty))
            } else {
                debug!(" => trivial drop glue");
                ty::InstanceDef::DropGlue(def_id, None)
            }
        } else {
            debug!(" => free item");
            // FIXME(effects): we may want to erase the effect param if that is present on this item.
            ty::InstanceDef::Item(def_id)
        };

        Ok(Some(Instance { def, args }))
    };
    debug!("resolve_instance: result={:?}", result);
    result
}

fn resolve_associated_item<'tcx>(
    tcx: TyCtxt<'tcx>,
    trait_item_id: DefId,
    param_env: ty::ParamEnv<'tcx>,
    trait_id: DefId,
    rcvr_args: GenericArgsRef<'tcx>,
) -> Result<Option<Instance<'tcx>>, ErrorGuaranteed> {
    debug!(?trait_item_id, ?param_env, ?trait_id, ?rcvr_args, "resolve_associated_item");

    let trait_ref = ty::TraitRef::from_method(tcx, trait_id, rcvr_args);

    let vtbl = match tcx.codegen_select_candidate((param_env, trait_ref)) {
        Ok(vtbl) => vtbl,
        Err(CodegenObligationError::Ambiguity) => {
            let reported = tcx.dcx().span_delayed_bug(
                tcx.def_span(trait_item_id),
                format!(
                    "encountered ambiguity selecting `{trait_ref:?}` during codegen, presuming due to \
                     overflow or prior type error",
                ),
            );
            return Err(reported);
        }
        Err(CodegenObligationError::Unimplemented) => return Ok(None),
        Err(CodegenObligationError::FulfillmentError) => return Ok(None),
    };

    // Now that we know which impl is being used, we can dispatch to
    // the actual function:
    Ok(match vtbl {
        traits::ImplSource::UserDefined(impl_data) => {
            debug!(
                "resolving ImplSource::UserDefined: {:?}, {:?}, {:?}, {:?}",
                param_env, trait_item_id, rcvr_args, impl_data
            );
            assert!(!rcvr_args.has_infer());
            assert!(!trait_ref.has_infer());

            let trait_def_id = tcx.trait_id_of_impl(impl_data.impl_def_id).unwrap();
            let trait_def = tcx.trait_def(trait_def_id);
            let leaf_def = trait_def
                .ancestors(tcx, impl_data.impl_def_id)?
                .leaf_def(tcx, trait_item_id)
                .unwrap_or_else(|| {
                    bug!("{:?} not found in {:?}", trait_item_id, impl_data.impl_def_id);
                });
            let infcx = tcx.infer_ctxt().build();
            let param_env = param_env.with_reveal_all_normalized(tcx);
            let args = rcvr_args.rebase_onto(tcx, trait_def_id, impl_data.args);
            let args = translate_args(
                &infcx,
                param_env,
                impl_data.impl_def_id,
                args,
                leaf_def.defining_node,
            );
            let args = infcx.tcx.erase_regions(args);

            // Since this is a trait item, we need to see if the item is either a trait default item
            // or a specialization because we can't resolve those unless we can `Reveal::All`.
            // NOTE: This should be kept in sync with the similar code in
            // `rustc_trait_selection::traits::project::assemble_candidates_from_impls()`.
            let eligible = if leaf_def.is_final() {
                // Non-specializable items are always projectable.
                true
            } else {
                // Only reveal a specializable default if we're past type-checking
                // and the obligation is monomorphic, otherwise passes such as
                // transmute checking and polymorphic MIR optimizations could
                // get a result which isn't correct for all monomorphizations.
                if param_env.reveal() == Reveal::All {
                    !trait_ref.still_further_specializable()
                } else {
                    false
                }
            };
            if !eligible {
                return Ok(None);
            }

            // HACK: We may have overlapping `dyn Trait` built-in impls and
            // user-provided blanket impls. Detect that case here, and return
            // ambiguity.
            //
            // This should not affect totally monomorphized contexts, only
            // resolve calls that happen polymorphically, such as the mir-inliner
            // and const-prop (and also some lints).
            let self_ty = rcvr_args.type_at(0);
            if !self_ty.is_known_rigid() {
                let predicates = tcx
                    .predicates_of(impl_data.impl_def_id)
                    .instantiate(tcx, impl_data.args)
                    .predicates;
                let sized_def_id = tcx.lang_items().sized_trait();
                // If we find a `Self: Sized` bound on the item, then we know
                // that `dyn Trait` can certainly never apply here.
                if !predicates.into_iter().filter_map(ty::Clause::as_trait_clause).any(|clause| {
                    Some(clause.def_id()) == sized_def_id
                        && clause.skip_binder().self_ty() == self_ty
                }) {
                    return Ok(None);
                }
            }

            // Any final impl is required to define all associated items.
            if !leaf_def.item.defaultness(tcx).has_value() {
                let guard = tcx.dcx().span_delayed_bug(
                    tcx.def_span(leaf_def.item.def_id),
                    "missing value for assoc item in impl",
                );
                return Err(guard);
            }

            let args = tcx.erase_regions(args);

            // Check if we just resolved an associated `const` declaration from
            // a `trait` to an associated `const` definition in an `impl`, where
            // the definition in the `impl` has the wrong type (for which an
            // error has already been/will be emitted elsewhere).
            if leaf_def.item.kind == ty::AssocKind::Const
                && trait_item_id != leaf_def.item.def_id
                && let Some(leaf_def_item) = leaf_def.item.def_id.as_local()
            {
                tcx.compare_impl_const((leaf_def_item, trait_item_id))?;
            }

            Some(ty::Instance::new(leaf_def.item.def_id, args))
        }
        traits::ImplSource::Builtin(BuiltinImplSource::Object { vtable_base }, _) => {
            traits::get_vtable_index_of_object_method(tcx, *vtable_base, trait_item_id).map(
                |index| Instance {
                    def: ty::InstanceDef::Virtual(trait_item_id, index),
                    args: rcvr_args,
                },
            )
        }
        traits::ImplSource::Builtin(BuiltinImplSource::Misc, _) => {
            let lang_items = tcx.lang_items();
            if Some(trait_ref.def_id) == lang_items.clone_trait() {
                // FIXME(eddyb) use lang items for methods instead of names.
                let name = tcx.item_name(trait_item_id);
                if name == sym::clone {
                    let self_ty = trait_ref.self_ty();
                    match self_ty.kind() {
                        ty::FnDef(..) | ty::FnPtr(_) => (),
                        ty::Coroutine(..)
                        | ty::CoroutineWitness(..)
                        | ty::Closure(..)
                        | ty::CoroutineClosure(..)
                        | ty::Tuple(..) => {}
                        _ => return Ok(None),
                    };

                    Some(Instance {
                        def: ty::InstanceDef::CloneShim(trait_item_id, self_ty),
                        args: rcvr_args,
                    })
                } else {
                    assert_eq!(name, sym::clone_from);

                    // Use the default `fn clone_from` from `trait Clone`.
                    let args = tcx.erase_regions(rcvr_args);
                    Some(ty::Instance::new(trait_item_id, args))
                }
            } else if Some(trait_ref.def_id) == lang_items.fn_ptr_trait() {
                if lang_items.fn_ptr_addr() == Some(trait_item_id) {
                    let self_ty = trait_ref.self_ty();
                    if !matches!(self_ty.kind(), ty::FnPtr(..)) {
                        return Ok(None);
                    }
                    Some(Instance {
                        def: ty::InstanceDef::FnPtrAddrShim(trait_item_id, self_ty),
                        args: rcvr_args,
                    })
                } else {
                    tcx.dcx().emit_fatal(UnexpectedFnPtrAssociatedItem {
                        span: tcx.def_span(trait_item_id),
                    })
                }
            } else if let Some(target_kind) = tcx.fn_trait_kind_from_def_id(trait_ref.def_id) {
                // FIXME: This doesn't check for malformed libcore that defines, e.g.,
                // `trait Fn { fn call_once(&self) { .. } }`. This is mostly for extension
                // methods.
                if cfg!(debug_assertions)
                    && ![sym::call, sym::call_mut, sym::call_once]
                        .contains(&tcx.item_name(trait_item_id))
                {
                    // For compiler developers who'd like to add new items to `Fn`/`FnMut`/`FnOnce`,
                    // you either need to generate a shim body, or perhaps return
                    // `InstanceDef::Item` pointing to a trait default method body if
                    // it is given a default implementation by the trait.
                    bug!(
                        "no definition for `{trait_ref}::{}` for built-in callable type",
                        tcx.item_name(trait_item_id)
                    )
                }
                match *rcvr_args.type_at(0).kind() {
                    ty::Closure(closure_def_id, args) => {
                        Some(Instance::resolve_closure(tcx, closure_def_id, args, target_kind))
                    }
                    ty::FnDef(..) | ty::FnPtr(..) => Some(Instance {
                        def: ty::InstanceDef::FnPtrShim(trait_item_id, rcvr_args.type_at(0)),
                        args: rcvr_args,
                    }),
                    ty::CoroutineClosure(coroutine_closure_def_id, args) => {
                        // When a coroutine-closure implements the `Fn` traits, then it
                        // always dispatches to the `FnOnce` implementation. This is to
                        // ensure that the `closure_kind` of the resulting closure is in
                        // sync with the built-in trait implementations (since all of the
                        // implementations return `FnOnce::Output`).
                        if ty::ClosureKind::FnOnce == args.as_coroutine_closure().kind() {
                            Some(Instance::new(coroutine_closure_def_id, args))
                        } else {
                            Some(Instance {
                                def: ty::InstanceDef::ConstructCoroutineInClosureShim {
                                    coroutine_closure_def_id,
                                    target_kind: ty::ClosureKind::FnOnce,
                                },
                                args,
                            })
                        }
                    }
                    _ => bug!(
                        "no built-in definition for `{trait_ref}::{}` for non-fn type",
                        tcx.item_name(trait_item_id)
                    ),
                }
            } else if let Some(target_kind) = tcx.async_fn_trait_kind_from_def_id(trait_ref.def_id)
            {
                match *rcvr_args.type_at(0).kind() {
                    ty::CoroutineClosure(coroutine_closure_def_id, args) => {
                        match (target_kind, args.as_coroutine_closure().kind()) {
                            (ClosureKind::FnOnce | ClosureKind::FnMut, ClosureKind::Fn)
                            | (ClosureKind::FnOnce, ClosureKind::FnMut) => {
                                // If we're computing `AsyncFnOnce`/`AsyncFnMut` for a by-ref closure,
                                // or `AsyncFnOnce` for a by-mut closure, then construct a new body that
                                // has the right return types.
                                //
                                // Specifically, `AsyncFnMut` for a by-ref coroutine-closure just needs
                                // to have its input and output types fixed (`&mut self` and returning
                                // `i16` coroutine kind).
                                Some(Instance {
                                    def: ty::InstanceDef::ConstructCoroutineInClosureShim {
                                        coroutine_closure_def_id,
                                        target_kind,
                                    },
                                    args,
                                })
                            }
                            _ => Some(Instance::new(coroutine_closure_def_id, args)),
                        }
                    }
                    ty::Closure(closure_def_id, args) => {
                        Some(Instance::resolve_closure(tcx, closure_def_id, args, target_kind))
                    }
                    ty::FnDef(..) | ty::FnPtr(..) => Some(Instance {
                        def: ty::InstanceDef::FnPtrShim(trait_item_id, rcvr_args.type_at(0)),
                        args: rcvr_args,
                    }),
                    _ => bug!(
                        "no built-in definition for `{trait_ref}::{}` for non-lending-closure type",
                        tcx.item_name(trait_item_id)
                    ),
                }
            } else {
                Instance::try_resolve_item_for_coroutine(tcx, trait_item_id, trait_id, rcvr_args)
            }
        }
        traits::ImplSource::Param(..)
        | traits::ImplSource::Builtin(BuiltinImplSource::TraitUpcasting { .. }, _)
        | traits::ImplSource::Builtin(BuiltinImplSource::TupleUnsizing, _) => None,
    })
}

pub(crate) fn provide(providers: &mut Providers) {
    *providers = Providers { resolve_instance, ..*providers };
}
