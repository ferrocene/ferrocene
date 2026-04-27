//! ## Recommended reading
//! - [Errors and lints](https://rustc-dev-guide.rust-lang.org/diagnostics.html)

use rustc_errors::{Applicability, Diag, MultiSpan};
use rustc_hir::def_id::{DefId, LOCAL_CRATE, LocalDefId};
use rustc_hir::{HirId, LangItem};
use rustc_middle::ty::TyCtxt;
use rustc_span::{STDLIB_STABLE_CRATES, Span};
use tracing::debug;

use crate::ferrocene::post_mono::InstantiationSite;
use crate::ferrocene::{
    KNOWN_UNVALIDATED, LintState, POSSIBLY_UNVALIDATED, UnvalidatedImplCause, Use, UseKind,
};

pub(super) fn lint_unvalidated_impl(
    tcx: TyCtxt<'_>,
    impl_item: LocalDefId,
    trait_decl: DefId,
    annotation: Span,
) {
    let lint_node = HirId::make_owner(impl_item);
    let impl_span = tcx.def_span(impl_item);
    let trait_span = annotation.until(tcx.def_span(trait_decl));

    tcx.emit_node_span_lint(
        KNOWN_UNVALIDATED,
        lint_node,
        impl_span,
        rustc_errors::DiagDecorator(|diag| {
            diag.primary_message("unvalidated implementation of validated trait");
            diag.span_note(trait_span, "trait marked as validated here");
            diag.note(format!(
                "callers of `{}` require every impl to be validated for safety",
                tcx.def_path_str(impl_item)
            ));
        }),
    );
}

/// Diagnostics.
impl<'tcx> LintState<'tcx> {
    fn func_span(&self, def_id: DefId) -> Span {
        match self.tcx.opt_item_ident(def_id) {
            Some(name) => name.span,
            None => self.tcx.def_span(def_id),
        }
    }

    pub(super) fn lint_use(&mut self, lint_node: HirId, use_: Use<'tcx>) {
        let Self { tcx, item: owner, .. } = *self;
        let (callee, receiver_span) = (use_.def_id(), use_.span);

        debug!("linting node {lint_node:?}");

        let pre_mono = matches!(use_.kind, UseKind::CalledPreMonoTraitFn(..));
        let lint = if pre_mono { POSSIBLY_UNVALIDATED } else { KNOWN_UNVALIDATED };

        tcx.emit_node_span_lint(lint, lint_node, receiver_span, rustc_errors::DiagDecorator(|diag| {
            let callee_descr = tcx.def_descr(callee);
            let owner_descr = tcx.def_descr(owner.into());
            diag.primary_message(format!(
                "validated {owner_descr} {} an unvalidated {callee_descr}",
                use_.present_tense()
            ));

            let added_suggestion = self.suggest_annotation(use_, diag);

            let name = match use_.opt_instance() {
                None => tcx.def_path_str(callee),
                Some(instance) => tcx.def_path_str_with_args(callee, instance.args),
            };
            let msg = format!("`{name}` is unvalidated");
            if added_suggestion {
                // do nothing for pre-mono functions -- we'll print "does not know" in a second,
                // and the added diagnostic isn't very helpful.
                if !pre_mono {
                    diag.note(msg);
                }
            } else {
                diag.span_label(self.func_span(callee), msg);
            }

            if let UseKind::ContainsFnPtr(_, ty) = use_.kind {
                diag.note(format!("`{name}` contains a function pointer that might be called at runtime"));
                diag.note(format!("the Ferrocene compiler does not know if the `{ty}` was verified, so it must assume it is unverified"));
            } else if let UseKind::CalledPreMonoTraitFn(trait_fn) = use_.kind {
                diag.note(format!("the Ferrocene compiler does not know if the implementation of `{}` will be validated or not", tcx.def_path_str(trait_fn)));
            }

            // Don't show this "takes place in a validated function" label more than once per function.
            // We really do need this as a separate bit of state from shown_lints because the lint might not be
            // emitted. ideally we would just `cancel` the diagnostic if we don't want to emit it,
            // but we don't get an owned `Diag` from `node_span_lint` :(
            if !self.shown_item {
                self.shown_item = true;
                let mut validated_span = MultiSpan::from_span(self.func_span(owner.into()));
                if let Some(annotation) = self.annotation {
                    validated_span.push_span_label(annotation, "marked as validated here");
                }

                self.decorate_cast(use_, diag);
                self.decorate_instantiation(use_, diag, Some(&mut validated_span));

                diag.span_note(
                    validated_span,
                    format!("`{}` is validated", tcx.def_path_str(owner)),
                );
                if self.annotation.is_none() {
                    diag.note("main functions are assumed to be validated");
                }
            } else {
                self.decorate_cast(use_, diag);
                self.decorate_instantiation(use_, diag, None);
            }
        }));
    }

    /// Returns whether this showed the `callee` span.
    fn suggest_annotation(&self, use_: Use<'tcx>, diag: &mut Diag<'_, ()>) -> bool {
        let tcx = self.tcx;
        let callee = use_.def_id();
        let callee_descr = tcx.def_descr(callee);

        let mut shown_span = false;

        let indent = |span: Span| {
            let col = tcx.sess.source_map().lookup_char_pos(span.lo()).col_display;
            " ".repeat(col)
        };

        let (suggestion_msg, attr) = if let UseKind::CalledPreMonoTraitFn(trait_fn) = use_.kind {
            let msg = format!(
                "if you know all implementations of this trait will be validated, mark `{}` as requiring validation",
                tcx.def_path_str(trait_fn)
            );
            (msg, "requires_validation")
        } else {
            (format!("consider marking the {callee_descr} as validated"), "prevalidated")
        };

        if callee.krate == LOCAL_CRATE {
            let header = tcx.def_span(callee);
            diag.span_suggestion_verbose(
                header.shrink_to_lo(),
                suggestion_msg,
                format!("#[ferrocene::{attr}]\n{}", indent(header)),
                Applicability::MaybeIncorrect,
            );
            shown_span = true;
        } else if STDLIB_STABLE_CRATES.contains(&tcx.crate_name(callee.krate)) {
            diag.help_once(format!(
                "contact Ferrocene support to see if this {callee_descr} is possible to certify"
            ));
        }

        if let UseKind::CalledPreMonoTraitFn(_) = use_.kind {
            let msg = "you can delay this warning until the compiler knows which implementation will be picked";
            let suggestion =
                format!("#[allow(ferrocene::possible_unvalidated)]\n{}", indent(use_.span));
            diag.span_suggestion(
                use_.span.shrink_to_lo(),
                msg,
                suggestion,
                Applicability::MaybeIncorrect,
            );
        }

        shown_span
    }

    fn decorate_cast(&self, use_: Use<'tcx>, diag: &mut Diag<'_, ()>) {
        let tcx = self.tcx;
        if matches!(use_.kind, UseKind::FnPtrCast(..)) {
            diag.note("once a function is cast to a function pointer, Ferrocene can no longer tell whether it is validated");
            diag.note("as a precaution, it must assume you will eventually call the function");
        } else if let UseKind::TraitObjectCast(cause, ty) = use_.kind {
            diag.note(format!("once `{ty}` is cast to a dynamic trait object, Ferrocene can no longer tell whether it is validated"));
            match cause {
                UnvalidatedImplCause::AssocFn(assoc_fn) => {
                    diag.note(format!(
                        "as a precaution, it must assume you will eventually call `{}`",
                        tcx.def_path_str(assoc_fn)
                    ));
                }
                UnvalidatedImplCause::UnresolvedGenericImpl(..) => {
                    unreachable!("all generics should be resolved by post-mono")
                }
            }
        }
    }

    fn decorate_instantiation(
        &self,
        use_: Use<'tcx>,
        diag: &mut Diag<'_, ()>,
        validated_span: Option<&mut MultiSpan>,
    ) {
        let tcx = self.tcx;
        if let Some(InstantiationSite {
            caller_span,
            caller_instance,
            pre_mono_callee,
            drop_fn,
            lint_node: _,
        }) = use_.from_instantiation
        {
            let caller_descr =
                tcx.def_path_str_with_args(caller_instance.def_id(), caller_instance.args);

            let drop = tcx.require_lang_item(LangItem::Drop, caller_span);
            let get_drop_impl = |def_id| {
                tcx.trait_impl_of_assoc(def_id).filter(|impl_| tcx.impl_trait_id(*impl_) == drop)
            };

            let msg = if let Some(impl_) =
                get_drop_impl(use_.def_id()).or(drop_fn.and_then(|drop| get_drop_impl(drop)))
            {
                let dropped_ty = tcx.type_of(impl_).skip_binder();
                // Call to drop(), injected by the compiler.
                format!("`{dropped_ty}` dropped here, in `{caller_descr}`")
            } else {
                let callee_descr = format!(
                    "generic {} `{}`",
                    tcx.def_descr(pre_mono_callee),
                    rustc_middle::ty::print::with_no_trimmed_paths!(
                        tcx.def_path_str(pre_mono_callee)
                    )
                );

                format!("{callee_descr} instantiated by `{caller_descr}`")
            };

            if let Some(multi) = validated_span {
                multi.push_span_label(caller_span, msg);
            } else {
                diag.span_note_once(
                    caller_span,
                    format!("{msg}, which is called from a validated entrypoint"),
                );
            }
        }
    }
}

impl<'tcx> Use<'tcx> {
    fn present_tense(self) -> &'static str {
        match self.kind {
            UseKind::Called(..) | UseKind::CalledPreMonoTraitFn(..) => "calls",
            // originally this said "type-erases" but that's unfamiliar jargon, and it's not clear
            // that it actually helps understanding.
            UseKind::TraitObjectCast(..) | UseKind::FnPtrCast(..) => "possibly calls",
            UseKind::ContainsFnPtr(..) => "uses",
        }
    }
}
