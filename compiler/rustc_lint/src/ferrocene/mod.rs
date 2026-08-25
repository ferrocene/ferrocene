//! This module contains the [`ferrocene::unvalidated`](UNVALIDATED) lint pass.
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
//! unvalidated item, which is fine as long as it's never actually sent to LLVM.
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
//! `uninstantiated::<i32>(0)`. Only then do we know whether the implementation is validated.
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
//! no longer have a way to retrieve its `#[ferrocene::prevalidated]` attribute. We want to avoid
//! having to ban function pointers altogether, so instead we force a decision of whether to lint at
//! the time of the cast. Consider this program:
//! ```rust
//! # #![feature(register_tool)] #![register_tool(ferrocene)]
//! fn unvalidated() {}
//! #[ferrocene::prevalidated]
//! fn returns_ptr() -> fn() { unvalidated } // not ok
//! ```
//! We have no idea whether some validated code is going to call `option.map(returns_ptr())`.
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
//! const PATH_MAX: usize = 2048;
//! let buffer = [0; PATH_MAX]; // totally fine
//! ```
//!
//! However, if there's a function pointer anywhere in the constant, we need to make sure that
//! function can't be called at runtime. In that case, we require the const or static to be marked
//! with `ferrocene::prevalidated` at each use site:
//!
//! ```
//! # use std::panic::{set_hook, PanicHookInfo};
//! fn unvalidated_panic(_: &PanicHookInfo) {}
//! const PANIC_HOOK: fn(&PanicHookInfo) = unvalidated_panic;
//! set_hook(Box::new(PANIC_HOOK)); //~ ERRROR PANIC_HOOK is unvalidated
//! ```
//!
//! Then, once the user adds the annotation, we walk the const body at the definition site.
//! ```
//! # use std::panic::{set_hook, PanicHookInfo};
//! fn unvalidated_panic(_: &PanicHookInfo) {}
//! #[ferrocene::prevalidated]
//! const PANIC_HOOK: fn(&PanicHookInfo) = unvalidated_panic;
//! //~^ ERROR unvalidated_panic is unvalidated
//! ```
//!
//! ### trait object coercions
//!
//! These are similar to function pointers, except trait objects bundle many function pointers
//! together, and determining which functions those actually are is non-trivial. See
//! [`LintState::check_dyn_trait_coercion`] for examples of how this works.
//!
//! ```
//! struct Unvalidated;
//! impl PartialEq<()> for Unvalidated {
//!     fn eq(&self, _: &()) -> bool { false }
//! }
//! // not ok: might call x.eq() later.
//! let x: &dyn PartialEq<()> = &Unvalidated;
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
//! generate LLVM IR for a given function. That allows us to assume that all function calls
//! can be resolved to an [`Instance`] (and error out otherwise). It also runs after ["elaborate
//! drops"](https://rustc-dev-guide.rust-lang.org/mir/drop-elaboration.html#drop-elaboration)
//! expands each drop to an explicit [`TerminatorKind::Drop`].
//!
//! *However*, it means we cannot depend on the function to be local to the current crate, or that
//! we have a lint node for the failing call, or that we have source spans or HIR available
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
//! struct S<T>(T);
//! impl<T: Default> S<T> { fn inherent() -> T { T::default() } }
//! ```
//! We get these type variables from a [`ParamEnv`].
//!
//! ## Recommended reading
//! - [Typing/parameter environments](https://rustc-dev-guide.rust-lang.org/typing-parameter-envs.html)
//! - [Monomorphization](https://rustc-dev-guide.rust-lang.org/backend/monomorph.html)

// NOTE: UNVALIDATED is public.
declare_tool_lint! {
    /// The `ferrocene::unvalidated` lint detects verified code that calls unverified functions.
    /// This may result in unverified code running in a safety critical context.
    ///
    /// This lint is a Ferrocene addition, and does not exist in upstream rustc.
    ///
    /// This lint is allowed-by-default, to avoid loud warnings for people using Ferrocene as a
    /// "normal" compiler. To enable it, add `#![warn(ferrocene::unvalidated)]` to each crate in
    /// your build, or add it to `[lints]` in Cargo.toml.
    pub ferrocene::UNVALIDATED,
    Allow,
    "a verified function called an unverified function",
    report_in_external_macro: true
}

// NOTE: LintUnvalidated is public.
declare_lint_pass!(LintUnvalidated => [UNVALIDATED]);

pub use post_mono::lint_validated_roots;

mod diagnostics;
mod dynamic_casts;
mod post_mono;
mod thir;

use rustc_data_structures::fx::FxHashSet;
use rustc_hir::def::DefKind;
use rustc_hir::{HirId, Item, TraitFn, TraitItem, TraitItemKind};
use rustc_middle::middle::codegen_fn_attrs::ferrocene::{
    ValidatedStatus, has_requires_validation_attribute, has_validated_attribute, item_is_validated,
};
use rustc_middle::span_bug;
use rustc_middle::ty::{Instance, Ty, TyCtxt};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_span::Span;
use rustc_span::def_id::{DefId, LocalDefId};
use tracing::{debug, info};

use crate::ferrocene::post_mono::InstantiationSite;
use crate::ferrocene::thir::LintThir;
use crate::{LateContext, LateLintPass};

// for intra-doc links
#[rustfmt::skip]
#[allow(unused_imports)]
use rustc_middle::{
    mir::TerminatorKind,
    ty::{GenericArgsRef, ParamEnv},
};

impl<'tcx> LateLintPass<'tcx> for LintUnvalidated {
    fn check_item_post(&mut self, cx: &LateContext<'tcx>, item: &Item<'tcx>) {
        check_attribute_placement(cx.tcx, item.owner_id.def_id, None);
        LintThir::check_item(cx.tcx, item.owner_id, item.owner_id.def_id);
    }

    fn check_trait_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx TraitItem<'tcx>) {
        check_attribute_placement(cx.tcx, item.owner_id.def_id, Some(item));
    }

    fn check_impl_item_post(
        &mut self,
        cx: &LateContext<'tcx>,
        item: &'tcx rustc_hir::ImplItem<'tcx>,
    ) {
        check_attribute_placement(cx.tcx, item.owner_id.def_id, None);
        check_impl_of_requires_validation(cx.tcx, item.owner_id.def_id);
        LintThir::check_item(cx.tcx, item.owner_id, item.owner_id.def_id);
    }
}

/// Reject `#[ferrocene::*]` attributes in positions where they have no defined
/// semantics, or are invalid.
///
/// `trait_item` is `Some` only if `def_id` is a trait item.
/// It is used to check if a method definition has a default body.
///
/// Emit hard errors rather than warnings, because a misplaced annotation is
/// always a mistake.
///
/// This is run when the item itself is checked, so the errors do not depend on
/// the item being called from a prevalidated function.
fn check_attribute_placement(
    tcx: TyCtxt<'_>,
    def_id: LocalDefId,
    trait_item: Option<&TraitItem<'_>>,
) {
    #[derive(Debug, PartialEq)]
    enum ItemRequiringValidation {
        ConstFnPtr {
            is_fn_ptr: bool,
            has_value: bool,
        },
        Method {
            default_body: bool,
        },
        /// Item does not require validation
        None,
    }

    let trait_method = match trait_item {
        Some(TraitItem { kind, .. }) => match kind {
            // Associated function
            TraitItemKind::Fn(_, body) => match body {
                TraitFn::Required(_) => ItemRequiringValidation::Method { default_body: false },
                TraitFn::Provided(_) => ItemRequiringValidation::Method { default_body: true },
            },
            // Associated constant
            TraitItemKind::Const(_ty, const_value) => {
                let const_ty = tcx.type_of(def_id.to_def_id()).skip_binder();
                debug!("Associated const has type `{const_ty:?}`");

                let has_value = const_value.is_some();
                let is_fn_ptr = thir::contains_unknown_fn(const_ty).is_some();

                ItemRequiringValidation::ConstFnPtr { has_value, is_fn_ptr }
            }
            // Associated type (unstable), with a concrete type
            TraitItemKind::Type(_bounds, Some(_assoc_ty)) => {
                let ty_ty = tcx.type_of(def_id.to_def_id()).skip_binder();
                debug!("Associated type has type `{ty_ty:?}`");

                let has_value = false;
                let is_fn_ptr = thir::contains_unknown_fn(ty_ty).is_some();

                ItemRequiringValidation::ConstFnPtr { has_value, is_fn_ptr }
            }
            // Associated type (unstable), without a concrete type
            // FIXME: should we check if the bound is any of the fn traits? Fn, FnMut, FnOnce
            TraitItemKind::Type(_bounds, None) => ItemRequiringValidation::None,
        },
        // Not a trait item
        None => ItemRequiringValidation::None,
    };
    let prevalidated = has_validated_attribute(tcx, def_id.to_def_id());
    let requires_validation = has_requires_validation_attribute(tcx, def_id.to_def_id());

    match (trait_method, prevalidated, requires_validation) {
        // `prevalidated` marks a body as validated, so it is meaningless on a
        // trait method definition without a default body
        (ItemRequiringValidation::Method { default_body: false }, Some(span), _) => {
            diagnostics::error_prevalidated_without_body(tcx, def_id, span)
        }
        // If a trait method has a default body and is marked with `requires_validation`,
        // it also has to be marked as `prevalidated`.
        (ItemRequiringValidation::Method { default_body: true }, None, Some(span)) => {
            diagnostics::error_requires_validation_without_prevalidated(tcx, def_id, span)
        }
        (ItemRequiringValidation::Method { .. }, None, None) => {
            // ok: method without any attribute
        }
        (ItemRequiringValidation::Method { default_body: true }, Some(_), None) => {
            // ok
        }
        (ItemRequiringValidation::Method { default_body: true }, Some(_), Some(_)) => {
            // ok
        }
        (ItemRequiringValidation::Method { default_body: false }, None, Some(_)) => {
            // ok
        }

        (ItemRequiringValidation::ConstFnPtr { is_fn_ptr: false, .. }, _, Some(_span)) => {
            // FIXME(error): const which are not fn ptr cannot be marked requires_validation
        }
        (ItemRequiringValidation::ConstFnPtr { is_fn_ptr: false, .. }, Some(_span), _) => {
            // FIXME(error): const which are not fn ptr cannot be marked prevalidated
            // FIXME: this and the case above could happen simultaneously, change it to be able to emit both together
        }
        (ItemRequiringValidation::ConstFnPtr { is_fn_ptr: false, .. }, None, None) => { /* ok */ }

        (
            ItemRequiringValidation::ConstFnPtr { is_fn_ptr: true, has_value: true },
            Some(_),
            Some(_),
        ) => {
            // ok: fn ptr const that has a value, is requires_validation and prevalidated
        }
        (
            ItemRequiringValidation::ConstFnPtr { is_fn_ptr: true, has_value: false },
            Some(_),
            None,
        ) => {
            // ok: fn ptr const that has no value, is requires_validation and not prevalidated
        }
        (ItemRequiringValidation::ConstFnPtr { is_fn_ptr: true, .. }, None, Some(_span)) => {
            // FIXME(error): const which is not required_validation cannot be marked prevalidated
        }
        (ItemRequiringValidation::ConstFnPtr { is_fn_ptr: true, has_value: true }, None, None) => {
            // ok
        }
        (
            ItemRequiringValidation::ConstFnPtr { is_fn_ptr: true, has_value: true },
            Some(_),
            None,
        ) => {
            // ok
        }
        (ItemRequiringValidation::ConstFnPtr { is_fn_ptr: true, has_value: false }, None, None) => {
            // ok
        }
        (
            ItemRequiringValidation::ConstFnPtr { is_fn_ptr: true, has_value: false },
            Some(_),
            Some(_),
        ) => {
            // FIXME(error): if no value it cannot be prevalidated
        }

        // `requires_validation` must be applied to trait methods or fn ptr consts only
        (ItemRequiringValidation::None, _, Some(span)) => {
            diagnostics::error_requires_validation_not_a_trait_fn(tcx, def_id, span)
        }
        (ItemRequiringValidation::None, Some(_), _) => {
            // FIXME(error): cannot mark unrelated item with prevalidated
        }
        (ItemRequiringValidation::None, None, None) => {
            // ok: no trait item which has neither attribute
        }
    }
}

/// Check that every implementation of a trait method marked with
/// `requires_validation` is marked with `prevalidated`.
///
/// A trait method that inherits the default implementation is not checked by
/// this. That is okay because `check_attribute_placement` guarantees that a
/// trait method with a default body that is marked with `requires_validation`
/// is also marked `prevalidated`.
fn check_impl_of_requires_validation(tcx: TyCtxt<'_>, impl_item: LocalDefId) {
    // Get the id of the trait method definition being implemented by `impl_item`,
    // if `impl_item` is an associated function with a body.
    let Some(trait_method_definition) = matches!(tcx.def_kind(impl_item), DefKind::AssocFn)
        // Inherited trait methods will be filtered out by `trait_item_of`
        .then(|| tcx.trait_item_of(impl_item.to_def_id()))
        .flatten()
    else {
        return;
    };

    if has_requires_validation_attribute(tcx, trait_method_definition).is_none() {
        // If the trait method definition has no `requires_validation`
        // attribute, there is nothing to check.
        return;
    } else if item_is_validated(tcx, impl_item.to_def_id()).validated() {
        // If the trait method implementation is `prevalidated` everything is fine
        return;
    } else {
        // If the trait method definition has a `requires_validation` attribute,
        // but the trait method implementation has no `prevalidated` attribute,
        // this is a problem! Therefore emit a lint!
        debug!("{impl_item:?} implements {trait_method_definition:?}, which requires validation");
        diagnostics::lint_impl_requires_validation(tcx, impl_item, trait_method_definition);
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
                // We don't care if types are unvalidated, only the functions that are called.
                DefKind::Struct | DefKind::Enum | DefKind::Union => {}
                kind => {
                    let item_span = tcx.def_span(item);
                    let span = match annotation {
                        Some(ref span) => span.with_hi(item_span.hi()),
                        None => item_span,
                    };
                    // FIXME: this should probably be `WARN unused attibute` instead?
                    span_bug!(span, "annotated validated with no body? {kind:?} {item:?}");
                }
            }
            debug!("ignoring validated item with no body: {item:?}");
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
            debug!("no need to lint call to validated {callee:?}");
            return;
        }

        // We have conditional logic below that -Z deduplicate-diagnostics doesn't know about.
        // Deduplicate lints manually.
        if tcx.sess.opts.unstable_opts.deduplicate_diagnostics && !self.shown_lints.insert(callee) {
            info!("ignoring duplicate lint for {callee:?}");
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

/// A use of an unvalidated item.
#[derive(Copy, Clone, Debug)]
struct Use<'tcx> {
    kind: UseKind<'tcx>,
    span: Span,
    from_instantiation: Option<InstantiationSite<'tcx>>,
}

#[derive(Copy, Clone, Debug)]
enum UnvalidatedImplCause<'tcx> {
    /// An associated function from the source type's impl of one of the traits we were casting to.
    ///
    /// FIXME(diagnostics): this should have all unvalidated items in the impl, not just the first.
    AssocFn(DefId),
    /// Only occurs pre-mono.
    UnresolvedGenericImpl(rustc_middle::ty::PolyTraitRef<'tcx>),
}

#[derive(Copy, Clone, Debug)]
enum UseKind<'tcx> {
    Called(Instance<'tcx>),
    FnPtrCast(Instance<'tcx>),
    /// The `Ty` is the source type of the cast. We don't currently store the destination type.
    TraitObjectCast(UnvalidatedImplCause<'tcx>, Ty<'tcx>),
    /// Only occurs for consts and statics.
    ContainsFnPtr(DefId, Ty<'tcx>),
}

impl<'tcx> Use<'tcx> {
    fn def_id(self) -> DefId {
        match self.kind {
            UseKind::Called(instance) | UseKind::FnPtrCast(instance) => instance.def_id(),
            UseKind::ContainsFnPtr(id, _) => id,
            UseKind::TraitObjectCast(UnvalidatedImplCause::AssocFn(id), _) => id,
            UseKind::TraitObjectCast(UnvalidatedImplCause::UnresolvedGenericImpl(trait_ref), _) => {
                trait_ref.def_id()
            }
        }
    }

    fn opt_instance(self) -> Option<Instance<'tcx>> {
        match self.kind {
            UseKind::FnPtrCast(instance) | UseKind::Called(instance) => Some(instance),
            UseKind::TraitObjectCast(..) | UseKind::ContainsFnPtr(..) => None,
        }
    }
}
