//! This module contains the [`ferrocene::uncertified`](UNCERTIFIED) lint pass.
//!
//! ## Architecture
//! There are two main passes: the [THIR pass](thir) and the
//! [post-monomorphization MIR pass](post_mono).
//! THIR runs on both `cargo check` and `cargo build`.
//! post-mono only runs on `cargo build`.
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
//! the time of the cast. Consider this program:
//! ```rust
//! fn unvalidated() {}
//! #[ferrocene::prevalidated]
//! fn returns_ptr() -> fn() { unvalidated } // not ok
//! ```
//! We have no idea whether some certified code is going to call `option.map(returns_ptr())`.
//! So we need to lint at the cast site instead.
//!
//! It might be possible to do fancy dataflow analysis to only disallow this if the pointer
//! "escapes" the current function, but that's complicated, and always checking at the cast site is
//! simple.
//!
//! ### const blocks
//!
//! Some function calls occur in the initializer of a `const` or `static`, not in a function body.
//! Usually this is totally fine: we argue to the assessor that compile-time code doesn't need to
//! (and can't) have line-coverage.
//!
//! ```
//! const PATH_MAX: i32 = 2048;
//! let buffer = [0; PATH_MAX]; // totally fine
//! ```
//!
//! However, if there's a function pointer anywhere in the
//! constant, we need to make sure that function can't be called at runtime. In that case, we
//! require the const or static to be marked with `ferrocene::prevalidated` at each use site, and
//! walk the const body at the definition site.
//!
//! ```
//! fn uncertified_panic(_: &PanicInfo) -> ! { loop {} }
//! const PANIC_HOOK: fn(&PanicInfo) -> ! = uncertified_panic;
//! set_panic(PANIC_HOOK); // not ok: uncertified_panic called at runtime
//! ```
//!
//! ### trait object coercions
//!
//! These are similar to function pointers, except trait objects bundle many function pointers
//! together, and determining which functions those actually are is non-trivial. See
//! [`LintThir::check_dyn_trait_coercion`] for examples of how this works.
//!
//! ```
//! struct Uncertified;
//! impl Clone for Uncertified { fn clone(&self) {} }
//! let x: &dyn Clone = &Uncertified; // not ok: might call x.clone() later.
//! ```
//!
//! ### THIR
//!
//! The THIR pass runs as a (mostly) standard [LateLintPass].
//! Unfortunately, LateLintPasses normally work on [HIR](https://rustc-dev-guide.rust-lang.org/hir.html)
//! *and* run near the end of compilation, which means that
//! [THIR](https://rustc-dev-guide.rust-lang.org/thir.html#the-thir) would normally not be
//! available. We preserve THIR all the way through the end of compilation, which causes Ferrocene
//! to use slightly more memory in exchange for getting better diagnostics.
//!
//! If the THIR pass cannot resolve an uninstantiated call (see "instantiations" above), it simply
//! silences the warning, assuming the post-mono pass will catch it.
//!
//! ### post-mono
//!
//! This pass is hacked into the
//! [`collect_and_partition_mono_items`](TyCtxt::collect_and_partition_mono_items)
//! [query](https://rustc-dev-guide.rust-lang.org/overview.html#queries), which runs on
//! [MIR](https://rustc-dev-guide.rust-lang.org/mir/index.html) just before the time we actually
//! generate LLVM IR for a given function. That allows us to assume that assume all function calls
//! can be resolved to an [`Instance`] (and error out otherwise). It also runs after ["elaborate
//! drops"](https://rustc-dev-guide.rust-lang.org/mir/drop-elaboration.html#drop-elaboration)
//! expands each drop to an explicit [`TerminatorKind::Drop`].
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
//! generic arguments were `[i32]`, which means our [`Instance`] was `uninstantiated::<i32>`.
//!
//! For our purposes, we care only about the instantiations of a function, not about any
//! declarations in a trait. In order to instantiate a function, we need to know both its
//! definition ([`DefId`]) and generic arguments
//! ([`GenericArgsRef`]). We may also need to resolve type
//! variables in scope. For example, in this program below, we cannot instantiate `inherent` unless
//! we know the type of `T` from the impl:
//! ```rust
//! struct S;
//! impl<T: Default> S { fn inherent() -> T { T::default() } }
//! ```
//! We get these type variables from a [`ParamEnv`].
//!
//! ## Recommended reading
//! - [Typing/parameter environments](https://rustc-dev-guide.rust-lang.org/typing-parameter-envs.html)
//! - [Monomorphization](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html)

// NOTE: UNCERTIFIED is public.
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

// NOTE: LintUncertified is public.
declare_lint_pass!(LintUncertified => [UNCERTIFIED]);

pub use post_mono::lint_validated_roots;

mod diagnostics;
mod post_mono;
mod thir;

// for intra-doc links
use rustc_data_structures::fx::FxHashSet;
use rustc_hir::def::DefKind;
use rustc_hir::{HirId, Item};
use rustc_middle::middle::codegen_fn_attrs::ferrocene::{ValidatedStatus, item_is_validated};
use rustc_middle::span_bug;
use rustc_middle::ty::{Instance, Ty, TyCtxt};
#[allow(unused_imports)]
use rustc_middle::{
    mir::TerminatorKind,
    ty::{GenericArgsRef, ParamEnv},
};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_span::Span;
use rustc_span::def_id::{DefId, LocalDefId};
use tracing::{debug, info};

use crate::ferrocene::post_mono::InstantiationSite;
use crate::ferrocene::thir::LintThir;
use crate::{LateContext, LateLintPass};

impl<'tcx> LateLintPass<'tcx> for LintUncertified {
    fn check_item_post(&mut self, cx: &LateContext<'tcx>, item: &Item<'tcx>) {
        LintThir::check_item(cx.tcx, item.owner_id, item.owner_id.def_id);
    }
}

struct LintState<'tcx> {
    tcx: TyCtxt<'tcx>,
    /// The item we are currently linting.
    item: LocalDefId,
    /// For diagnostics; used to point to the `#[ferrocene::prevalidated]` attribute.
    annotation: Option<Span>,
    /// For diagnostics; see [`lint_use`](LintState::lint_use).
    shown_item: bool,
    /// For deduplication; see [`check_use`](LintState::check_use).
    shown_lints: FxHashSet<DefId>,
}

impl<'tcx> LintState<'tcx> {
    /// Check whether `item` needs to be linted at all. If so, return a new `LintState`.
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

    /// Check whether an item use needs to be linted. If so, lint it.
    fn check_use(&mut self, lint_node: HirId, use_: Use<'tcx>) {
        let tcx = self.tcx;
        let callee = use_.def_id();

        if matches!(item_is_validated(tcx, callee), ValidatedStatus::Validated { .. }) {
            debug!("no need to lint call to certified {callee:?}");
            return;
        }

        // We have conditional logic below that -Z deduplicate-diagnostics doesn't know about.
        // Deduplicate lints manually.
        if tcx.sess.opts.unstable_opts.deduplicate_diagnostics && !self.shown_lints.insert(callee) {
            info!(r#"ignoring duplicate lint for {callee:?}"#);
            return;
        }

        self.lint_use(lint_node, use_);
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

impl<'tcx> InstantiateResult<'tcx> {
    fn instance(self) -> Option<Instance<'tcx>> {
        match self {
            InstantiateResult::Err | InstantiateResult::Indeterminate => None,
            InstantiateResult::Resolved(instance) => Some(instance),
        }
    }
}

/// A use of an uncertified item.
#[derive(Copy, Clone, Debug)]
struct Use<'tcx> {
    kind: UseKind<'tcx>,
    span: Span,
    from_instantiation: Option<InstantiationSite<'tcx>>,
}

#[derive(Copy, Clone, Debug)]
enum UseKind<'tcx> {
    Called(Instance<'tcx>),
    FnPtrCast(Instance<'tcx>),
    TraitObjectCast(DefId, Ty<'tcx>),
    /// Only occurs for consts and statics.
    ContainsFnPtr(DefId, Ty<'tcx>),
    Named(DefId),
}

impl<'tcx> Use<'tcx> {
    fn def_id(self) -> DefId {
        match self.kind {
            UseKind::Called(instance) | UseKind::FnPtrCast(instance) => instance.def_id(),
            UseKind::Named(id) | UseKind::ContainsFnPtr(id, _) => id,
            UseKind::TraitObjectCast(assoc_fn, _) => assoc_fn,
        }
    }

    fn opt_instance(self) -> Option<Instance<'tcx>> {
        match self.kind {
            UseKind::FnPtrCast(instance) | UseKind::Called(instance) => Some(instance),
            UseKind::TraitObjectCast(..) | UseKind::ContainsFnPtr(..) | UseKind::Named(..) => None,
        }
    }
}
