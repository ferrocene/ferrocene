//! This module contains the `ferrocene::uncertified` lint pass.
//!
//! ## Architecture
//! There are two main passes: the THIR pass and the post-monomorphization MIR pass.
//!
//! The THIR pass is exclusively for diagnostics; our soundness argument does not rely on it at all.
//! It only exists because it sucks to only see errors multiple crates later than they happened,
//! especially for highly generic crates like core.
//!
//! The post-mono pass only runs on code that has been monomorphized for codegen.
//! In particular, it only runs on reachable code; it's very possible to have dead code that uses an
//! uncertified item, which is fine as long as it's never actually sent to LLVM.
//! In most cases, but not all, this will be caught by the THIR pass.
//!
//! ### instantiations
//!
//! We need a post-mono pass because we may not be able to resolve all function calls immediately.
//! Consider this program:
//! ```rust
//! fn uninstantiated<T: Clone>(x: T) { x.clone(); }
//! ```
//! At the time we first see it, we have no idea what the type of T is, so we cannot resolve
//! `<T as Clone>::clone`. We have to wait until we see a caller that monomorphizes it as (e.g.)
//! `uninstantiated::<i32>(0)`. Only then do we know whether the implementation is verified.
//!
//! ### macros
//!
//! Because we depend on this lint for our validity argument, we report the lint even through
//! external macros; just because a macro was defined in core does not mean the functions it calls
//! are validated.
//!
//! ### function pointers
//!
//! Normally we only lint at call sites. However, once a function is cast to a function pointer, we
//! no longer have a way to retrieve its `#[ferrocene::prevalidated]` attributes. We want to avoid
//! having to ban function pointers altogether, so instead we force a decision of whether to lint at
//! the time of the cast.
//!
//! ### const blocks
//!
//! Some function calls occur in the initializer of a `const` or `static`, not in a function body.
//! For the most part this doesn't cause any issues; we just walk these bodies like any other.
//! The only hitch is that, as a result, we need to check each use of a `const` or `static` to make
//! sure that it's validated.
//!
//! ### THIR
//!
//! The THIR pass runs as a (mostly) standard LateLintPass.
//! Unfortunately, LateLintPasses normally work on HIR *and* run near the end of compilation, which
//! means that THIR would normally not be available. We preserve THIR all the way through the end of
//! compilation, which causes Ferrocene to use slightly more memory in exchange for getting better
//! diagnostics.
//!
//! If the THIR pass cannot resolve an uninstantiated call (see "instantiations" above), it simply
//! silences the warning, assuming the post-mono pass will catch it.
//!
//! ### post-mono
//!
//! This pass is hacked into the `check_mono_item` query, which runs just before the time we
//! actually generate LLVM IR for a given function. That allows us to assume that assume all
//! function calls can be resolved to an instance (and error out otherwise).
//!
//! *However*, it means we cannot depend on the function to be local to the current crate, or that
//! we have a lint node for the failing call, or that we have we have source spans or HIR available
//! for the failing call.
//!
//! This sucks a lot! What we do instead is look at the *caller* of the unvalidated function.
//! For example, in our example above, our lint is on the `uninstantiated(0_i32)` call, not the
//! `x.clone` call. We show `x.clone` as the primary span, but our decision of whether or not to
//! emit the lint comes from the `uninstantiated()` call.
//!
//! ## Implementation
//!
//! First, some background on Rust's type system and compilation model.
//! Each function in a Rust program is only *defined* in one place, but it may be *instantiated*
//! many times with different generic arguments. Our definition above was `fn uninstantiated`, and our
//! generic arguments were `[i32]`, which means our `Instance` was `uninstantiated::<i32>`.
//!
//! For our purposes, we care only about the instantiations of a function, not about any
//! declarations in a trait. In order to instantiate a function, we need to know both its definition
//! (`DefId`) and generic arguments (`GenericArgsRef`). We may also need to resolve type variables
//! in scope. For example, in this program below, we cannot instantiate `inherent` unless we know
//! the type of `T` from the impl:
//! ```rust
//! struct S;
//! impl<T: Default> S { fn inherent() -> T { T::default() } }
//! ```
//! We get these type variables from a `ParamEnv`.

use std::ops::ControlFlow;

use rustc_data_structures::fx::FxHashSet;
use rustc_errors::{Diag, MultiSpan};
use rustc_hir::def::DefKind;
use rustc_hir::{CRATE_HIR_ID, HirId, Item, OwnerId};
use rustc_middle::middle::codegen_fn_attrs::ferrocene::{ValidatedStatus, item_is_validated};
use rustc_middle::mir::mono::MonoItem;
use rustc_middle::mir::visit::Visitor as _;
use rustc_middle::mir::{
    self, Body, CastKind, ClearCrossCrate, Location, Rvalue, SourceInfo, Terminator, TerminatorKind,
};
use rustc_middle::span_bug;
use rustc_middle::thir::visit::Visitor as _;
use rustc_middle::thir::{self, Thir};
use rustc_middle::ty::adjustment::PointerCoercion;
use rustc_middle::ty::{
    self, Binder, EarlyBinder, ExistentialPredicate, ExistentialTraitRef, GenericArgs,
    GenericArgsRef, Instance, InstanceKind, Ty, TyCtxt, TypeFoldable, TypeSuperVisitable,
    TypeVisitable, TypeVisitor, TypingEnv,
};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_span::def_id::{DefId, LocalDefId};
use rustc_span::{STDLIB_STABLE_CRATES, Span};
use tracing::{debug, info, trace};

use crate::{LateContext, LateLintPass};

declare_tool_lint! {
    /// The `ferrocene::uncertified` lint detects verified code that calls unverified functions.
    /// This is not allowed if you want your code to be certified by a safety assessor.
    ///
    /// This lint is a Ferrocene addition, and does not exist in upstream rustc.
    pub ferrocene::UNCERTIFIED,
    /// This lint is allowed-by-default, to avoid loud warnings for people using ferrocene as a
    /// "normal" compiler. To enable it, add `#![warn(ferrocene::uncertified)]` to each crate in
    /// your build, or add it to `[lints]` in Cargo.toml.
    Allow,
    "a verified function called an unverified function",
    report_in_external_macro: true
}

declare_lint_pass!(LintUncertified => [UNCERTIFIED]);

impl<'tcx> LateLintPass<'tcx> for LintUncertified {
    fn check_item_post(&mut self, cx: &LateContext<'tcx>, item: &Item<'tcx>) {
        LintThir::lint_owner(cx.tcx, item.owner_id, item.owner_id.def_id);
    }
}

struct LintState<'tcx> {
    tcx: TyCtxt<'tcx>,
    item: LocalDefId,
    annotation: Option<Span>,
    shown_item: bool,
    shown_lints: FxHashSet<DefId>,
}

struct LintThir<'thir, 'tcx> {
    thir: &'thir Thir<'tcx>,
    linter: LintState<'tcx>,
    owner: OwnerId,
}

struct LintPostMono<'a, 'tcx> {
    instance: Instance<'tcx>,
    linter: &'a mut LintState<'tcx>,
    visited: &'a mut FxHashSet<Instance<'tcx>>,
    body: &'a Body<'tcx>,
    from_instantiation: Option<InstantiationSite<'tcx>>,
}

struct InstantiationSite<'tcx> {
    /// NOTE: this points to the call site which causes the callee to be monomorphized.
    lint_node: HirId,
    caller_span: Span,
    caller_instance: Instance<'tcx>,
}

impl<'tcx> LintState<'tcx> {
    fn new(tcx: TyCtxt<'tcx>, item: LocalDefId) -> Option<Self> {
        let ValidatedStatus::Validated { annotation } = item_is_validated(tcx, item.into()) else {
            return None;
        };

        if tcx.hir_node_by_def_id(item).associated_body().is_none() {
            match tcx.def_kind(item) {
                // TODO: do we need to check if an ADT uses an unvalidated type?
                DefKind::Struct | DefKind::Enum | DefKind::Union => {}
                kind => {
                    let item_span = tcx.def_span(item);
                    let span = match annotation {
                        Some(ref span) => span.with_hi(item_span.hi()),
                        None => item_span,
                    };
                    span_bug!(span, "annotated certified with no body? {kind:?} {item:?}");
                }
            }
            debug!("ignoring certified item with no body: {item:?}");
            return None;
        }

        debug!("check {item:?}");
        Some(LintState {
            tcx,
            item,
            annotation,
            shown_item: false,
            shown_lints: FxHashSet::default(),
        })
    }

    fn func_span(&self, def_id: DefId) -> Span {
        match self.tcx.opt_item_ident(def_id) {
            Some(name) => name.span,
            None => self.tcx.def_span(def_id),
        }
    }

    fn lint(
        &mut self,
        lint_node: HirId,
        use_: UseKind<'tcx>,
        extra_info: impl FnOnce(&mut Diag<'_, ()>, Option<&mut MultiSpan>),
    ) {
        let Self { tcx, item: owner, .. } = *self;

        let (callee, receiver_span) = use_.as_parts();

        if matches!(item_is_validated(tcx, callee), ValidatedStatus::Validated { .. }) {
            debug!("no need to lint call to certified {callee:?}");
            return;
        }

        // We have conditional logic below that -Z deduplicate-diagnostics doesn't know about.
        // Deduplicate lints manually.
        if tcx.sess.opts.unstable_opts.deduplicate_diagnostics && !self.shown_lints.insert(callee) {
            info!("ignoring duplicate lint for {callee:?}");
            return;
        }

        debug!("linting node {lint_node:?}");
        tcx.node_span_lint(UNCERTIFIED, lint_node, receiver_span, |diag| {
            let callee_descr = tcx.def_descr(callee);
            let owner_descr = tcx.def_descr(owner.into());
            diag.primary_message(format!(
                "validated {owner_descr} {} an unvalidated {callee_descr}",
                use_.present_tense()
            ));

            // Need to do this lazily or `with_no_trimmed_paths` will panic :/
            let name = match use_ {
                UseKind::Named(_, _) | UseKind::ContainsTy(..) => tcx.def_path_str(callee),
                UseKind::Cast(instance, _) | UseKind::Called(instance, _) => {
                    tcx.def_path_str_with_args(callee, instance.args)
                }
            };
            // TODO: this is horrible for `Cast` or `ContainsTy`. Need to rewrite all this
            // logic to bring the appropriate data into `lint` and then actually look at
            // the discriminant instead of just collapsing it to `name`.
            diag.span_label(self.func_span(callee), format!("`{name}` is unvalidated"));

            if matches!(use_, UseKind::ContainsTy(..)) {
                diag.note(format!("`{name}` contains a function pointer that might be called at runtime"));
                diag.note("the Ferrocene compiler does not know if that function was verified, so it must assume it is unverified");
            }

            if STDLIB_STABLE_CRATES.contains(&tcx.crate_name(callee.krate)) {
                diag.help_once(format!(
                    "contact Ferrocene support to see if this {callee_descr} is possible to certify"
                ));
            }

            // Don't show this "takes place in a certified function" label more than once per function.
            // We really do need this as a separate bit of state from shown_lints because the lint might not be
            // emitted. ideally we would just `cancel` the diagnostic if we don't want to emit it,
            // but we don't get an owned `Diag` from `node_span_lint` :(
            if !self.shown_item {
                self.shown_item = true;
                let mut validated_span = MultiSpan::from_span(self.func_span(owner.into()));
                if let Some(annotation) = self.annotation {
                    validated_span.push_span_label(annotation, "marked as validated here");
                }
                extra_info(diag, Some(&mut validated_span));

                diag.span_note(
                    validated_span,
                    format!("`{}` is validated", tcx.def_path_str(owner)),
                );
                if self.annotation.is_none() {
                    diag.note("main functions are assumed to be validated");
                }
            } else {
                extra_info(diag, None);
            }
        });
    }
}

// Lint all used items recursively, starting from validated roots.
// Validated roots are calculated in `rustc_monomorphize::collector::ferrocene`, see there for
// details.
pub fn lint_validated_roots<'tcx>(tcx: TyCtxt<'tcx>, roots: Vec<MonoItem<'tcx>>) {
    trace!("all roots: {roots:?}");

    let mut visited = FxHashSet::default();
    // TODO: reuse linter across roots so we don't emit duplicate diagnostics.
    // let linter = LintHelper::new(tcx, local);
    for root in roots {
        let instance = match root {
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
                    continue;
                }
                instance
            }
        };
        debug!("linting root: {instance:?}");
        let def_id = instance.def_id().expect_local();
        if let Some(mut linter) = LintState::new(tcx, def_id) {
            LintPostMono::visit_instance(&mut linter, &mut visited, instance, None);
        }
    }
}

impl<'a, 'tcx> mir::visit::Visitor<'tcx> for LintPostMono<'a, 'tcx> {
    fn visit_terminator(&mut self, terminator: &Terminator<'tcx>, location: Location) {
        if let Some((callee_instance, pre_mono_call)) = self.get_call_def_mir(terminator, location)
        {
            let use_ = UseKind::Called(callee_instance, terminator.source_info.span);
            self.on_edge(use_, &terminator.source_info, pre_mono_call, |_| ());
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

        let Some((pre_mono_call, generic_args)) = operand.const_fn_def() else {
            span_bug!(
                call_span,
                "don't know how to handle ReifyFnPointer cast of non-constant fn {operand:?}"
            );
        };

        let callee_instance = self.monomorphize_instance(pre_mono_call, generic_args, call_span);
        // TODO: need to also check this in THIR pass
        let use_ = UseKind::Cast(callee_instance, call_span);
        let source_info = self.body.source_info(location);
        self.on_edge(use_, source_info, pre_mono_call, |diag| {
            diag.note("once a function is cast to a function pointer, Ferrocene can no longer tell whether it is validated");
            diag.note("as a precaution, it must assume you will eventually call the function");
        });
    }
}

impl<'a, 'tcx> LintPostMono<'a, 'tcx> {
    fn on_edge(
        &mut self,
        use_: UseKind<'tcx>,
        source_info: &SourceInfo,
        pre_mono_call: DefId,
        decorate: impl for<'d> FnOnce(&mut Diag<'d, ()>),
    ) {
        let (callee_instance, call_span) = use_.expect_instance();

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
                Some(local) => local.lint_node,
                // A local root can resolve to a cross-crate instantiation when a MIR inline pass runs.
                // We don't have anywhere to point to, so just point to the crate root.
                None => CRATE_HIR_ID,
            },
        };

        self.lint_at(lint_node, use_, pre_mono_call, decorate);

        let site =
            InstantiationSite { lint_node, caller_instance: self.instance, caller_span: call_span };
        trace!("recurse into {callee_instance:?}");
        LintPostMono::visit_instance(self.linter, self.visited, callee_instance, Some(site));
    }

    fn lint_at(
        &mut self,
        lint_node: HirId,
        use_: UseKind<'tcx>,
        pre_mono_call: DefId,
        decorate: impl for<'d> FnOnce(&mut Diag<'d, ()>),
    ) {
        let caller = match self.from_instantiation {
            Some(InstantiationSite { caller_span, caller_instance, .. }) => {
                Some((caller_span, caller_instance))
            }
            None => {
                // TODO: i think this is wrong? it's assuming THIR will catch all issues in the
                // current crate, but i'm not sure that's true ...
                info!("ignoring root instantiation of {use_:?}");
                return;
                // info!("linting root instantiation of {use_:?}");
                // None
            }
        };

        let tcx = self.linter.tcx;
        let (callee_instance, _) = use_.expect_instance();

        let callee = callee_instance.def_id();
        let lint_item = self.linter.item;

        // This is a bit odd - we use the HIR id of the caller function,
        // not the callee that actually caused the error.
        // This is so people can `allow` individual instantiations rather than having to
        // blanket-allow all of them.
        self.linter.lint(lint_node, use_, |diag: &mut Diag<'_, _>, validated_span| {
            decorate(diag);

            let callee_descr = format!(
                "generic {} `{}`",
                tcx.def_descr(callee),
                rustc_middle::ty::print::with_no_trimmed_paths!(tcx.def_path_str(pre_mono_call))
            );

            if let Some((caller_span, caller_instance)) = caller {
                let msg = format!(
                    "{callee_descr} instantiated by `{}`",
                    tcx.def_path_str_with_args(caller_instance.def_id(), caller_instance.args)
                );

                if let Some(multi) = validated_span {
                    multi.push_span_label(caller_span, msg);
                } else {
                    diag.span_note(
                        caller_span,
                        format!("{msg}, which is called from a validated entrypoint"),
                    );
                }
            } else {
                diag.note(format!(
                    "{callee_descr} instantiated by validated entrypoint {}",
                    tcx.def_path_str(lint_item)
                ));
            }
        });
    }

    fn visit_instance(
        linter: &'a mut LintState<'tcx>,
        visited: &mut FxHashSet<Instance<'tcx>>,
        instance: Instance<'tcx>,
        from_instantiation: Option<InstantiationSite<'tcx>>,
    ) {
        let tcx = linter.tcx;
        let owner = instance.def_id();

        if !tcx.is_mir_available(owner) {
            // We've already compiled this item in a previous crate and we didn't save the
            // MIR between crates.
            // We must have checked the item when it was compiled, so just ignore it.
            info!("no MIR for {owner:?}");
            return;
        } else if !visited.insert(instance) {
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
        let mut this = LintPostMono { linter, visited, instance, body, from_instantiation };
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
                        // pointers we can get came from a certified function.
                        ty::FnPtr(..) => {}
                        _ => {
                            // If this is anything other than a function item, it can't have generics and
                            // therefore must have been checked by the THIR pass.
                            // TODO: are we sure is this true when we're passed an `impl Fn`?
                            tcx.dcx()
                                .span_delayed_bug(span, format!("called a non-function? {func:?}"));
                        }
                    }
                    info!("ignoring call to non-constant function {func:?}");
                    return None;
                };
                let instance = self.monomorphize_instance(pre_mono_call, generic_args, span);
                (pre_mono_call, instance)
            }
            TerminatorKind::Drop { place, target: _, unwind: _, replace: _, drop, async_fut } => {
                if drop.is_some() || async_fut.is_some() {
                    span_bug!(span, "ferrocene::certified doesn't know how to check async drop");
                }

                let drop_in_place = tcx.lang_items().drop_in_place_fn().unwrap();
                let (ty, _) = self.monomorphize_args(place.ty(self.body, tcx), span);
                let generics = tcx.mk_args(&[ty.ty.into()]);
                // Use DropGlue directly rather than going through drop_in_place so that we get
                // better spans.
                let def = InstanceKind::DropGlue(drop_in_place, Some(ty.ty));
                let instance = Instance { def, args: generics };
                debug!("resolve drop glue => instance={instance:?}, ty={ty:?}");
                (drop_in_place, instance)
            }
            _ => return None,
        };

        // if call_instance.def_id() == pre_mono_call
        //     && generic_args.is_empty()
        //     && pre_mono_call.is_local()
        //     && tcx.dcx().err_count() == 0
        // {
        // TODO: this is broken if the warning isn't set to Deny, lol
        // tcx.dcx().span_delayed_bug(span, format!("THIR pass didn't catch simple call? {pre_mono_call:?}"));
        // }

        Some((call_instance, pre_mono_call))
    }

    fn monomorphize_instance(
        &self,
        def_id: DefId,
        generic_args: GenericArgsRef<'tcx>,
        span: Span,
    ) -> Instance<'tcx> {
        let (mono_args, typing_env) = self.monomorphize_args(generic_args, span);
        Instance::expect_resolve(self.linter.tcx, typing_env, def_id, mono_args, span)
    }

    fn monomorphize_args<T>(
        &self,
        generic_args: T,
        span: Span,
    ) -> (T, TypingEnv<'tcx>)
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

impl<'thir, 'tcx: 'thir> thir::visit::Visitor<'thir, 'tcx> for LintThir<'thir, 'tcx> {
    fn thir(&self) -> &'thir Thir<'tcx> {
        self.thir
    }

    fn visit_expr(&mut self, expr: &'thir thir::Expr<'tcx>) {
        let use_ = match self.get_call_def_thir(expr) {
            None => return,
            Some(use_) => use_,
        };
        let hir_id = HirId { owner: self.owner, local_id: expr.temp_scope_id };
        debug!("id={hir_id:?}, kind={:?}", expr.kind);
        self.linter.lint(hir_id, use_, |_, _| ());
    }
}

fn dyn_trait<'tcx>(ty: Ty<'tcx>, span: Span) -> Vec<Binder<'tcx, ExistentialTraitRef<'tcx>>> {
    struct ContainsDynTraitVisitor(Span);

    impl<'tcx> TypeVisitor<TyCtxt<'tcx>> for ContainsDynTraitVisitor {
        type Result = ControlFlow<Vec<Binder<'tcx, ExistentialTraitRef<'tcx>>>>;

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
                                // We already require the implementation of the trait to be certified
                                // (that's the `Trait` predicate below), so which impl gets picked
                                // doesn't matter as long we know which one it is.
                                ExistentialPredicate::Projection(_) => None,
                                ExistentialPredicate::Trait(t) => Some(t),
                            })
                            .transpose();
                        if let Some(t) = trait_ {
                            traits.push(t);
                        }
                    }
                    // TODO: is it possible to have multiple Dynamic types in a single top-level
                    // type? how? maybe with an enum?
                    ControlFlow::Break(traits)
                }
                _ => t.super_visit_with(self),
            }
        }
    }

    let cf = ty.visit_with(&mut ContainsDynTraitVisitor(span));
    cf.break_value().unwrap_or_default()
}

/// c.f. Ty::contains_closure
fn contains_unknown_fn<'tcx>(ty: Ty<'tcx>) -> Option<Ty<'tcx>> {
    struct ContainsUnknownFnVisitor;

    impl<'tcx> TypeVisitor<TyCtxt<'tcx>> for ContainsUnknownFnVisitor {
        type Result = ControlFlow<Ty<'tcx>>;

        fn visit_ty(&mut self, t: Ty<'tcx>) -> Self::Result {
            match t.kind() {
                ty::Dynamic(..) | ty::FnPtr(_, _) => ControlFlow::Break(t),
                _ => t.super_visit_with(self),
            }
        }
    }

    let cf = ty.visit_with(&mut ContainsUnknownFnVisitor);
    cf.break_value()
}

impl<'thir, 'tcx: 'thir> LintThir<'thir, 'tcx> {
    fn lint_owner(tcx: TyCtxt<'tcx>, owner: OwnerId, item: LocalDefId) -> Option<()> {
        if tcx.sess.opts.test
            && tcx.entry_fn(()).and_then(|(id, _)| id.as_local()) == Some(owner.def_id)
        {
            // We don't lint `main` functions if they're a shim generated by the `--test` machinery.
            info!("treating libtest main function as unvalidated");
            return None;
        }

        let linter = LintState::new(tcx, item)?;
        // thir_body can return ErrorGuaranteed if this is a const block that failed evaluation.
        let body = tcx.thir_body(item).ok()?;
        let thir = &body.0.borrow();
        let mut visitor = LintThir { linter, thir, owner };
        for expr in &*thir.exprs {
            visitor.visit_expr(expr);
        }

        Some(())
    }

    fn get_call_def_thir(&mut self, expr: &thir::Expr<'tcx>) -> Option<UseKind<'tcx>> {
        let tcx = self.linter.tcx;

        let def_id = match expr.kind {
            thir::ExprKind::NamedConst { def_id, .. }
            | thir::ExprKind::StaticRef { def_id, .. } => {
                if let Some(unknown_fn) = contains_unknown_fn(expr.ty) {
                    // It's possible for runtime code to access an unknown function type from this
                    // contant. Ensure that it's marked with `prevalidated` so that its body gets
                    // checked.
                    return Some(UseKind::ContainsTy(def_id, unknown_fn, expr.span));
                } else {
                    // These have bodies, but they are always evaluated at compile time.
                    // We argue to our assessor that means that the correct behavior is
                    // validated whenever the const/static is used in a runtime function, so the
                    // functions that generate the constant don't need to be tested separately.
                    // The constants themselves execute no code at runtime, so mentioning them is ok.
                    return None;
                }
            }
            thir::ExprKind::ZstLiteral { .. } => match expr.ty.kind() {
                ty::FnDef(maybe_trait_fn, generic_args) => {
                    debug!("saw zst {:?}", expr.ty);
                    self.get_concrete_fn_def(*maybe_trait_fn, generic_args, expr.span)?.def_id()
                }
                _ => span_bug!(expr.span, "called ZST literals should always be named functions"),
            },
            // we use a custom span here, not the expr's span, so return immediately.
            thir::ExprKind::Call { ty, .. } => {
                let diag_span = tcx.sess.source_map().span_until_char(expr.span, '(');
                let instance = self.instance_of_ty(ty, expr.span)?;

                debug!("saw call to {instance:?}");
                return Some(UseKind::Called(instance, diag_span));
            }
            // We assume all closure definitions in this function are also certified.
            // However, we still need to check the closure body to make sure it doesn't call
            // uncertified functions.
            thir::ExprKind::Closure(ref expr) => {
                // Closures are never an owner, so we need to hang onto the original owner so that
                // our synthesized HirIds are valid.
                LintThir::lint_owner(tcx, self.owner, expr.closure_id);
                return None;
            }
            thir::ExprKind::PointerCoercion { cast: PointerCoercion::Unsize, .. } => {
                let bound_traits = dyn_trait(expr.ty, expr.span);
                for trait_ in bound_traits {
                    for assoc_id in tcx.associated_item_def_ids(trait_.def_id()) {
                        if tcx.def_kind(assoc_id).is_fn_like() {
                            // TODO: this is a giant hack and not even right lmao
                            // in particular it says "not certified" when it should do the
                            // same thing as fn pointers and say "the compiler can't
                            // guarantee this is right".
                            debug!("saw unsized coercion to {:?}", expr.ty);
                            return Some(UseKind::Named(*assoc_id, expr.span));
                        }
                    }
                }
                return None;
            }
            // Nothing to check.
            _ => return None,
        };
        Some(UseKind::Named(def_id, expr.span))
    }

    fn instance_of_ty(&self, ty: Ty<'tcx>, span: Span) -> Option<Instance<'tcx>> {
        match ty.kind() {
            ty::FnDef(maybe_trait_fn, generic_args) => {
                let callee = self.get_concrete_fn_def(*maybe_trait_fn, generic_args, span)?;
                Some(callee)
            }
            // Reference to a function or function pointer.
            ty::Ref(_, ty, _) => self.instance_of_ty(*ty, span),
            // We assume that all functions pointers are valid. Proof:
            // 1. If the function was certified, no problem.
            // 2. If the function was uncertified, and is a literal or assigned to a local
            //    variable, then either:
            //    - We can resolve it to a concrete instance, in which case we would have caught it in ZstLiteral above.
            //    - We can't resolve it yet, but it stays a unique function type, so we will
            //    catch the call later in the post-mono pass.
            //    - We can't resolve it yet and it's cast to a function pointer so we don't
            //    have enough info to catch it post-mono when it's called. In this case we
            //    catch it in `ReifyFnPtr` above.
            // 3. If the function was passed as an argument, then either:
            //   - We were called by an uncertified function. No problem.
            //   - We were called by a certified function. This lint will run on that
            //   function too, and we will catch it there at the time it is checked /
            //   monomorphized.
            // 4. If this is a closure then either:
            //   - It was defined in this function, in which case we treat it as also
            //   certified.
            //   - It was passed as an argument, which is ok by 3).
            //   - It is a global const/static, so we catch it in NamedConst/StaticRef above.
            ty::FnPtr(..) => None,
            other => span_bug!(span, "unsupported call kind {other:?}"),
        }
    }

    fn get_concrete_fn_def(
        &self,
        maybe_trait_fn: DefId,
        generic_args: GenericArgsRef<'tcx>,
        span: Span,
    ) -> Option<Instance<'tcx>> {
        let callee = match self.try_instantiate(maybe_trait_fn, generic_args, span) {
            InstantiateResult::Err => return None,
            // This is handled later by a post-mono pass that checks the instantiation is verified.
            // For now just ignore errors.
            InstantiateResult::Indeterminate => return None,
            InstantiateResult::Resolved(id) => id,
        };
        // If this is a call to Fn/FnMut/FnOnce::call, then it's a type that implements
        // `fn_traits`, either a closure or a user-defined type. Allow closures but check
        // everything else.
        if self.linter.tcx.is_closure_like(callee.def_id()) {
            debug!("skipping closure {callee:?}");
            return None;
        }

        // Skip trait functions. These happen when we're calling the vtable of a `dyn` unsized
        // object. This case is caught below in `PointerCoercion::Unsize`.
        if matches!(callee.def, InstanceKind::Virtual(..)) {
            info!("skipping dyn assoc item {callee:?}");
            return None;
        }

        Some(callee)
    }

    fn try_instantiate(
        &self,
        def_id: DefId,
        args: &'tcx GenericArgs<'tcx>,
        span: Span,
    ) -> InstantiateResult<'tcx> {
        let tcx = self.linter.tcx;

        use rustc_middle::ty::TypingMode;

        // apparently the rest of THIR uses this? it has a comment saying it's wrong though...
        //let typing_mode = TypingMode::non_body_analysis();
        // this definitely works but there's scary comments about revealing opaque types
        // let env = TypingEnv::post_analysis(tcx, self.linter.owner);

        let typing_mode = TypingMode::typeck_for_body(tcx, self.linter.item);
        let param_env = tcx.param_env(self.linter.item);
        let env = TypingEnv { typing_mode, param_env };

        match Instance::try_resolve(tcx, env, def_id, args) {
            Err(_) => { // this happens when we hit the type length limit
                tcx.dcx().span_delayed_bug(span, format!("could not resolve instance ({def_id:?}, {args:?})"));
                InstantiateResult::Err
            }
            Ok(None) => InstantiateResult::Indeterminate,
            Ok(Some(instance)) => InstantiateResult::Resolved(instance),
        }
    }
}

#[derive(Debug)]
enum InstantiateResult<'tcx> {
    /// Compilation is going to fail anyway. No need to do anything fancy.
    Err,
    /// We found the instance.
    Resolved(Instance<'tcx>),
    /// We don't yet have enough info to resolve this to a concrete function.
    Indeterminate,
}

#[derive(Copy, Clone, Debug)]
enum UseKind<'tcx> {
    Called(Instance<'tcx>, Span),
    Named(DefId, Span),
    Cast(Instance<'tcx>, Span),
    ContainsTy(DefId, Ty<'tcx>, Span),
}

impl<'tcx> UseKind<'tcx> {
    fn as_parts(self) -> (DefId, Span) {
        match self {
            Self::Called(instance, span) | Self::Cast(instance, span) => (instance.def_id(), span),
            Self::Named(id, span) | Self::ContainsTy(id, _, span) => (id, span),
        }
    }

    #[track_caller]
    fn expect_instance(self) -> (Instance<'tcx>, Span) {
        match self {
            UseKind::Cast(instance, span) | UseKind::Called(instance, span) => (instance, span),
            UseKind::ContainsTy(..) | UseKind::Named(..) => unreachable!(), // only in THIR pass
        }
    }

    fn present_tense(self) -> &'static str {
        match self {
            UseKind::Called(..) => "calls",
            UseKind::Named(..) => "uses",
            // originally this said "type-erases" but that's unfamiliar jargon, and it's not clear
            // that it actually helps understanding.
            UseKind::Cast(..) => "possibly calls",
            UseKind::ContainsTy(..) => "uses",
        }
    }
}
