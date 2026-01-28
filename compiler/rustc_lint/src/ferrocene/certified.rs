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

use rustc_data_structures::fx::FxHashSet;
use rustc_errors::{Diag, MultiSpan};
use rustc_hir::{HirId, Item};
use rustc_hir::def::DefKind;
use rustc_middle::middle::codegen_fn_attrs::ferrocene::{ValidatedStatus, item_is_validated};
use rustc_middle::span_bug;
use rustc_middle::ty::{Instance, Ty, TyCtxt};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_span::def_id::{DefId, LocalDefId};
use rustc_span::{STDLIB_STABLE_CRATES, Span};
use tracing::{debug, info};

use crate::ferrocene::thir::LintThir;
use crate::{LateContext, LateLintPass};

declare_tool_lint! {
    /// The `ferrocene::uncertified` lint detects verified code that calls unverified functions.
    /// This is not allowed if you want your code to be certified by a safety assessor.
    ///
    /// This lint is a Ferrocene addition, and does not exist in upstream rustc.
    ///
    /// This lint is allowed-by-default, to avoid loud warnings for people using ferrocene as a
    /// "normal" compiler. To enable it, add `#![warn(ferrocene::uncertified)]` to each crate in
    /// your build, or add it to `[lints]` in Cargo.toml.
    pub ferrocene::UNCERTIFIED,
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

pub(super) struct LintState<'tcx> {
    pub(super) tcx: TyCtxt<'tcx>,
    pub(super) item: LocalDefId,
    annotation: Option<Span>,
    shown_item: bool,
    shown_lints: FxHashSet<DefId>,
}

impl<'tcx> LintState<'tcx> {
    pub(super) fn new(tcx: TyCtxt<'tcx>, item: LocalDefId) -> Option<Self> {
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

    pub(super) fn lint(
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

#[derive(Debug)]
pub(super) enum InstantiateResult<'tcx> {
    /// Compilation is going to fail anyway. No need to do anything fancy.
    Err,
    /// We found the instance.
    Resolved(Instance<'tcx>),
    /// We don't yet have enough info to resolve this to a concrete function.
    Indeterminate,
}

#[derive(Copy, Clone, Debug)]
pub(super) enum UseKind<'tcx> {
    Called(Instance<'tcx>, Span),
    Named(DefId, Span),
    Cast(Instance<'tcx>, Span),
    ContainsTy(DefId, Ty<'tcx>, Span),
}

impl<'tcx> UseKind<'tcx> {
    pub(super) fn as_parts(self) -> (DefId, Span) {
        match self {
            Self::Called(instance, span) | Self::Cast(instance, span) => (instance.def_id(), span),
            Self::Named(id, span) | Self::ContainsTy(id, _, span) => (id, span),
        }
    }

    #[track_caller]
    pub(super) fn expect_instance(self) -> (Instance<'tcx>, Span) {
        match self {
            UseKind::Cast(instance, span) | UseKind::Called(instance, span) => (instance, span),
            UseKind::ContainsTy(..) | UseKind::Named(..) => unreachable!(), // only in THIR pass
        }
    }

    pub(super) fn present_tense(self) -> &'static str {
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
