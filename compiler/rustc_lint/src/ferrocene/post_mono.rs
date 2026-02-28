//! Run a post-mono pass on MIR, possibly from other crates.
//! In post-mono MIR, all functions are possible to resolve to an [`Instance`].
//!
//! This traverses the callgraph along function call edges, starting from mono roots, and stopping
//! if it sees a function that's already been checked. See `rustc_monomorphize::collector` for more
//! info.
//!
//! A "mono root" is an externally reachable function, such as `main`, `pub fn`, or weird things
//! such as `std::rt::lang_start`.
//!
//! There are three functions in play during this pass.
//! 1. The unvalidated function being called, which we call the 'callee'.
//! 2. The generic function being instantiated, *which may not be in the current crate*, called the
//!    'caller'.
//! 3. The functions that instantiated it (recursively, back to the mono root), which we call the
//!    [`InstantiationSite`].
//!
//! ## Recommended reading
//! - [MIR Debugging](https://rustc-dev-guide.rust-lang.org/mir/debugging.html)

use rustc_data_structures::fx::FxHashSet;
use rustc_data_structures::unord::UnordSet;
use rustc_hir::def_id::DefId;
use rustc_hir::{CRATE_HIR_ID, HirId};
use rustc_middle::mir::mono::MonoItem;
use rustc_middle::mir::visit::Visitor as _;
use rustc_middle::mir::{
    self, Body, CastKind, ClearCrossCrate, Location, Rvalue, SourceInfo, Terminator, TerminatorKind,
};
use rustc_middle::span_bug;
use rustc_middle::ty::adjustment::PointerCoercion;
use rustc_middle::ty::{
    self, EarlyBinder, GenericArgsRef, Instance, InstanceKind, TyCtxt, TypeFoldable, TypingEnv,
};
use rustc_span::Span;
use tracing::{debug, info, trace};

use crate::ferrocene::{LintState, Use, UseKind};

struct LintPostMono<'a, 'tcx> {
    /// The function we are currently traversing.
    instance: Instance<'tcx>,
    /// Its body.
    body: &'a Body<'tcx>,
    linter: &'a mut LintState<'tcx>,
    /// A list of all functions we have previously traversed.
    /// This needs to store Instances, not DefIds, because different instantiations may call
    /// different concrete functions, and we want to make sure we lint all of them.
    visited: &'a mut FxHashSet<Instance<'tcx>>,
    /// A list of all items we are going to traverse.
    /// This is needed to avoid non-determinism in diagnostics; we don't want `from_instantiation`
    /// to vary based on iteration order.
    roots: &'a [MonoItem<'tcx>],
    /// May be `None` if this is a mono root.
    from_instantiation: Option<InstantiationSite<'tcx>>,
}

/// Used for diagnostics. See [`post_mono`](self).
#[derive(Copy, Clone, Debug)]
pub(super) struct InstantiationSite<'tcx> {
    /// NOTE: this points to the call site which causes the callee to be monomorphized.
    pub(super) lint_node: HirId,
    pub(super) caller_span: Span,
    pub(super) caller_instance: Instance<'tcx>,
    /// `callee_instance`, but before we called `expect_instance` on it.
    /// This may not be the same as `use_.def_id()` if we resolved an associated function
    /// to an implementation.
    pub(super) pre_mono_callee: DefId,
    pub(super) drop_fn: Option<DefId>,
}

/// Lint all used items recursively, starting from validated roots.
/// Validated roots are calculated in `rustc_monomorphize::collector::ferrocene`, see there for
/// details.
///
/// We can't depend on anything in rustc_monomorphize here because we're too early in [rustc's
/// dependency graph](https://rustc-dev-guide.rust-lang.org/compiler-src.html#big-picture). Instead
/// we call this function in a query override in `rustc_interface`.
pub fn lint_validated_roots<'tcx>(tcx: TyCtxt<'tcx>, roots: UnordSet<MonoItem<'tcx>>) {
    trace!("all roots: {roots:?}");

    let mut visited = FxHashSet::default();

    // We need to sort these for query stability.
    let roots = tcx.with_stable_hashing_context(move |ref hcx| roots.into_sorted(hcx, true));

    // FIXME: reuse linter across roots so we don't emit duplicate diagnostics.
    // let linter = LintHelper::new(tcx, local);
    for root in &roots {
        let instance = match *root {
            // global asm is always an exported constraint
            MonoItem::GlobalAsm(..) => continue,
            // NOTE: `mono` panics if item has generics rather than silently doing the wrong thing
            MonoItem::Static(def_id) => Instance::mono(tcx, def_id),
            MonoItem::Fn(instance) => {
                let def = instance.def_id();

                // Skip std::rt::lang_start. Technically we could lint on it as if it were the span
                // of `main`, but the lint would never be useful.
                // In general we treat all shims as part of the compiler qualification rather than
                // the standard library certification, since they're only accessible through
                // language features.
                // Note that we may not have `lang_start` yet if we're still compiling core.
                if Some(def) == tcx.lang_items().start_fn() {
                    continue;
                } else if !def.is_local() && Some(def) == tcx.entry_fn(()).map(|(id, _)| id) {
                    // it's possible to have main functions that came from another crate!
                    // FIXME: do we need to lint this somehow?
                    // i think main is required to be fully monomorphic so we would have checked it
                    // when compiling the dependency?
                    continue;
                }
                instance
            }
        };
        debug!("linting root: {instance:?}");
        let def_id = instance.def_id().expect_local();
        if let Some(mut linter) = LintState::new(tcx, def_id) {
            LintPostMono::visit_instance(&mut linter, &mut visited, &roots, instance, None);
        }
    }
}

impl<'a, 'tcx> mir::visit::Visitor<'tcx> for LintPostMono<'a, 'tcx> {
    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        if let Some((callee_instance, pre_mono_call)) = self.get_call_def_mir(terminator, location)
        {
            let use_ = self.use_(UseKind::Called(callee_instance), terminator.source_info.span);
            self.on_edge(use_, &terminator.source_info, pre_mono_call);
        }
    }

    fn visit_rvalue(&mut self, rval: &Rvalue<'tcx>, location: Location) {
        let Rvalue::Cast(
            CastKind::PointerCoercion(PointerCoercion::ReifyFnPointer(_), _),
            operand,
            _fn_ptr_ty,
        ) = rval
        else {
            return;
        };
        let call_span = operand.span(self.body);

        let Some((pre_mono_callee, generic_args)) = operand.const_fn_def() else {
            span_bug!(
                call_span,
                "don't know how to handle ReifyFnPointer cast of non-constant fn {operand:?}"
            );
        };

        let callee_instance = self.monomorphize_instance(pre_mono_callee, generic_args, call_span);
        // FIXME: want to also check this in THIR pass
        let use_ = self.use_(UseKind::FnPtrCast(callee_instance), call_span);
        let source_info = self.body.source_info(location);
        self.on_edge(use_, source_info, pre_mono_callee);
    }
}

impl<'a, 'tcx> LintPostMono<'a, 'tcx> {
    fn use_(&self, kind: UseKind<'tcx>, span: Span) -> Use<'tcx> {
        Use { kind, span, from_instantiation: self.from_instantiation }
    }

    fn on_edge(&mut self, use_: Use<'tcx>, source_info: &SourceInfo, pre_mono_callee: DefId) {
        let callee_instance = use_.expect_instance();

        // Recurse into the instantiated call. Keep the call span for diagnostics.
        // Try to update the lint node if possible, but use the lint node of the caller if the
        // callee is cross-crate.
        // FIXME: we have enough info here to show a backtrace of how the function was instantiated,
        // maybe pass that in so we can show it?
        let lint_node = match self.body.source_scopes[source_info.scope].local_data.as_ref() {
            ClearCrossCrate::Set(data) => data.lint_root,
            // We assume that all roots come from the current crate.
            // This is checked earlier in `lint_validated_roots`.
            ClearCrossCrate::Clear => match self.from_instantiation.as_ref() {
                // This is a bit odd - we use the HIR id of the caller function,
                // not the callee that actually caused the error.
                // The callee is in another crate so we don't have any choice here.
                Some(local) => local.lint_node,
                // A local root can resolve to a cross-crate instantiation when a MIR inline pass runs.
                // We don't have anywhere to point to, so just point to the crate root.
                None => CRATE_HIR_ID,
            },
        };

        self.linter.check_use(lint_node, use_);

        let site =
            if Some(self.instance.def_id()) == self.linter.tcx.lang_items().drop_in_place_fn() {
                // We want to show a better span; drop_in_place is never interesting since the body is
                // synthesized by a MIR shim anyway.
                // Note that we saw it, though, so diagnostics can say "dropped here".
                InstantiationSite {
                    drop_fn: Some(callee_instance.def_id()),
                    ..self.from_instantiation.unwrap()
                }
            } else {
                InstantiationSite {
                    drop_fn: None,
                    lint_node,
                    caller_instance: self.instance,
                    caller_span: use_.span,
                    pre_mono_callee,
                }
            };

        if self.roots.contains(&MonoItem::Fn(callee_instance)) {
            info!("don't need to recurse into {callee_instance:?}, we'll lint it separately");
            return;
        }

        trace!("recurse into {callee_instance:?}, lint_node={lint_node:?}");
        LintPostMono::visit_instance(
            self.linter,
            self.visited,
            self.roots,
            callee_instance,
            Some(site),
        );
    }

    fn visit_instance(
        linter: &'a mut LintState<'tcx>,
        visited: &mut FxHashSet<Instance<'tcx>>,
        roots: &'a [MonoItem<'tcx>],
        mut instance: Instance<'tcx>,
        from_instantiation: Option<InstantiationSite<'tcx>>,
    ) {
        let tcx = linter.tcx;
        let owner = instance.def_id();

        if let Some(intrinsic) = tcx.intrinsic(owner) {
            if intrinsic.must_be_overridden {
                // Instrinsics with no fallback body are qualified as part of the compiler,
                // and will panic in `instance_mir`.
                info!("ignoring intrinsic {owner:?}");
                return;
            }

            // See equivalent code in `rustc_monomorphize::collector::visit_instance_use`.
            if tcx.sess.replaced_intrinsics.contains(&intrinsic.name) {
                // This is normal: LLVM in particular has specialized overrides for many
                // integer operations. But that means the fallback body won't actually be
                // used, so don't try to check it.
                info!("ignoring overridden intrinsic body for {owner:?}");
                return;
            }

            debug!("using fallback body for {owner:?}");
            instance = ty::Instance::new_raw(owner, instance.args);
        } else if !tcx.is_mir_available(owner) {
            // We've already compiled this item in a previous crate and we didn't save the
            // MIR between crates.
            // We must have checked the item when it was compiled, so just ignore it.
            info!("no MIR for {owner:?}");
            return;
        }

        if !visited.insert(instance) {
            // We've already linted this instance (or maybe we're still halfway through linting it).
            // Don't loop forever.
            //
            // NOTE: this means that `-Z deduplicate-diagnostics=no` doesn't work properly for
            // post-mono errors. I think this isn't worth fixing; just use separate test files if
            // you need to test the same instance being instantiated more than once.
            //
            // NOTE: because of the funny way we calculate lint nodes, this means that if the same
            // item is instantiated in multiple places, only the lint level of the first
            // instantiation will be respected. It might be possible to fix this by caching the
            // lint level in addition to the instance itself?
            info!("already linted {instance:?}");
            return;
        }

        let body = tcx.instance_mir(instance.def);
        let mut this = LintPostMono { linter, visited, roots, instance, body, from_instantiation };
        for (bb, data) in mir::traversal::reachable(body) {
            this.visit_basic_block_data(bb, data);
        }
    }

    fn get_call_def_mir(
        &self,
        terminator: &Terminator<'tcx>,
        _loc: Location,
    ) -> Option<(Instance<'tcx>, DefId)> {
        let tcx = self.linter.tcx;
        let span = terminator.source_info.span;

        let (pre_mono_call, call_instance) = match &terminator.kind {
            TerminatorKind::Call { func, .. } | TerminatorKind::TailCall { func, .. } => {
                let Some((pre_mono_call, generic_args)) = func.const_fn_def() else {
                    match func.ty(self.body, tcx).kind() {
                        kind @ ty::FnDef(..) => {
                            span_bug!(span, "{kind:?} should have been a const_fn_def?")
                        }
                        // ok: see reasoning in THIR pass, we have checks to ensure all function
                        // pointers we can get came from a validated function.
                        ty::FnPtr(..) => {}
                        _ => {
                            // If this is anything other than a function item, it can't have generics and
                            // therefore must have been checked by the THIR pass.
                            // FIXME: are we sure is this true when we're passed an `impl Fn`?
                            tcx.dcx()
                                .span_delayed_bug(span, format!("called a non-function? {func:?}"));
                        }
                    }
                    info!("ignoring call to non-constant function {func:?}");
                    return None;
                };
                let mut instance = self.monomorphize_instance(pre_mono_call, generic_args, span);
                if matches!(instance.def, InstanceKind::Virtual(..)) {
                    // This is a call through a vtable: (x as dyn Trait).foo().
                    // We don't know what instance `foo` resolves too, but we linted earlier when
                    // `x` was cast to `dyn Trait`, so we can assume this call here is ok.
                    // See the reasoning in THIR about function pointers.
                    return None;
                }

                // Look for `<T as Fn>::call` for some T.
                // If T is a function type, the compiler synthesizes an impl, so we'll check
                // the trait declaration in libcore which isn't what we want. Check if the
                // function is annotated instead.
                if let InstanceKind::FnPtrShim(_, ty) | InstanceKind::FnPtrAddrShim(_, ty) =
                    instance.def
                    && let ty::FnDef(fn_item, fn_args) = ty.kind()
                {
                    debug!("found function item {fn_item:?}");
                    instance = self.monomorphize_instance(*fn_item, fn_args, span);
                }

                (pre_mono_call, instance)
            }
            TerminatorKind::Drop { place, target: _, unwind: _, replace: _, drop, async_fut } => {
                if drop.is_some() || async_fut.is_some() {
                    span_bug!(span, "ferrocene::validated doesn't know how to check async drop");
                }

                let (ty, env) = self.monomorphize_args(place.ty(self.body, tcx));
                if !ty.ty.needs_drop(tcx, env) {
                    return None; // otherwise instance_mir panics
                }

                let drop_in_place = tcx.lang_items().drop_in_place_fn().unwrap();
                let generics = tcx.mk_args(&[ty.ty.into()]);
                // We can't use DropGlue directly because `create_coroutine_drop_shim` treats
                // coroutines specially and we'll crash if we try to avoid going through it.
                // Instead we skip drop_in_place when iterating roots, it's never interesting and
                // we want to show a different instantiation site in diagnostics.
                let def = InstanceKind::Item(drop_in_place);
                let instance = Instance { def, args: generics };
                debug!("resolve drop glue => instance={instance:?}, ty={ty:?}");
                (drop_in_place, instance)
            }
            _ => return None,
        };

        Some((call_instance, pre_mono_call))
    }

    fn monomorphize_instance(
        &self,
        def_id: DefId,
        generic_args: GenericArgsRef<'tcx>,
        span: Span,
    ) -> Instance<'tcx> {
        let (mono_args, typing_env) = self.monomorphize_args(generic_args);
        Instance::expect_resolve(self.linter.tcx, typing_env, def_id, mono_args, span)
    }

    fn monomorphize_args<T>(&self, generic_args: T) -> (T, TypingEnv<'tcx>)
    where
        T: TypeFoldable<TyCtxt<'tcx>>,
    {
        let tcx = self.linter.tcx;

        let env = TypingEnv::post_analysis(tcx, self.linter.item);
        let args = self.instance.instantiate_mir_and_normalize_erasing_regions(
            tcx,
            env,
            EarlyBinder::bind(generic_args),
        );
        (args, env)
    }
}

impl<'tcx> Use<'tcx> {
    #[track_caller]
    fn expect_instance(self) -> Instance<'tcx> {
        self.opt_instance().unwrap_or_else(|| {
            span_bug!(self.span, "called expect_instance on a THIR-only lint kind")
        })
    }
}
