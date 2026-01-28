use rustc_errors::{Diag, MultiSpan};
use rustc_middle::middle::codegen_fn_attrs::ferrocene::{item_is_validated, ValidatedStatus};
use rustc_hir::{HirId, def_id::DefId};
use rustc_span::{STDLIB_STABLE_CRATES, Span};
use tracing::{debug, info};

use crate::ferrocene::{UNCERTIFIED, certified::{LintState, UseKind}};

impl<'tcx> LintState<'tcx> {
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
            info!(r#"ignoring duplicate lint for {callee:?}"#);
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

impl<'tcx> UseKind<'tcx> {
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
