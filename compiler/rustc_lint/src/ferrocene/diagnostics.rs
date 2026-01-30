//! ## Recommended reading
//! - [Errors and lints](https://rustc-dev-guide.rust-lang.org/diagnostics.html)

use rustc_errors::{Diag, MultiSpan};
use rustc_hir::HirId;
use rustc_hir::def_id::DefId;
use rustc_span::{STDLIB_STABLE_CRATES, Span};
use tracing::debug;

use crate::ferrocene::post_mono::InstantiationSite;
use crate::ferrocene::{LintState, UNCERTIFIED, Use, UseKind};

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
        tcx.node_span_lint(UNCERTIFIED, lint_node, receiver_span, |diag| {
            let callee_descr = tcx.def_descr(callee);
            let owner_descr = tcx.def_descr(owner.into());
            diag.primary_message(format!(
                "validated {owner_descr} {} an unvalidated {callee_descr}",
                use_.present_tense()
            ));

            // Need to do this lazily or `with_no_trimmed_paths` will panic :/
            let name = match use_.opt_instance() {
                None => tcx.def_path_str(callee),
                Some(instance) => tcx.def_path_str_with_args(callee, instance.args),
            };
            diag.span_label(self.func_span(callee), format!("`{name}` is unvalidated"));

            if let UseKind::ContainsFnPtr(_, ty) = use_.kind {
                diag.note(format!("`{name}` contains a function pointer that might be called at runtime"));
                diag.note(format!("the Ferrocene compiler does not know if the `{ty}` was verified, so it must assume it is unverified"));
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
        });
    }

    fn decorate_cast(&self, use_: Use<'tcx>, diag: &mut Diag<'_, ()>) {
        let tcx = self.tcx;
        if matches!(use_.kind, UseKind::FnPtrCast(..)) {
            diag.note("once a function is cast to a function pointer, Ferrocene can no longer tell whether it is validated");
            diag.note("as a precaution, it must assume you will eventually call the function");
        } else if let UseKind::TraitObjectCast(assoc_fn, ty) = use_.kind {
            if let Some(trait_) = tcx.trait_of_assoc(assoc_fn) {
                diag.note(format!(
                    "Ferrocene cannot tell whether `{ty}`'s implementation of `{}` is validated",
                    tcx.def_path_str(trait_)
                ));
                diag.note(format!(
                    "as a precaution, it must assume `{}` is unvalidated",
                    tcx.def_path_str(assoc_fn)
                ));
            }
            diag.note("once an object is cast to a dynamic trait object, Ferrocene can no longer tell whether it is validated");
            diag.note(format!(
                "as a precaution, it must assume you will eventually call `{}`",
                tcx.def_path_str(assoc_fn)
            ));
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
            pre_mono_item,
            lint_node: _,
        }) = use_.from_instantiation
        {
            let callee_descr = format!(
                "generic {} `{}`",
                tcx.def_descr(pre_mono_item),
                rustc_middle::ty::print::with_no_trimmed_paths!(tcx.def_path_str(pre_mono_item))
            );

            let msg = format!(
                "{callee_descr} instantiated by `{}`",
                tcx.def_path_str_with_args(caller_instance.def_id(), caller_instance.args)
            );

            if let Some(multi) = validated_span {
                multi.push_span_label(caller_span, msg);
            } else {
                diag.span_note_once(
                    caller_span,
                    format!("{msg}, which is called from a validated entrypoint"),
                );
            }
            // } else {
            //     diag.note(format!(
            //             "{callee_descr} instantiated by validated entrypoint {}",
            //             tcx.def_path_str(self.item)
            //     ));
        }
    }
}

impl<'tcx> Use<'tcx> {
    pub(super) fn present_tense(self) -> &'static str {
        match self.kind {
            UseKind::Called(..) => "calls",
            // originally this said "type-erases" but that's unfamiliar jargon, and it's not clear
            // that it actually helps understanding.
            UseKind::TraitObjectCast(..) | UseKind::FnPtrCast(..) => "possibly calls",
            UseKind::ContainsFnPtr(..) => "uses",
        }
    }
}
