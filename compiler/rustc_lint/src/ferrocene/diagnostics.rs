//! ## Recommended reading
//! - [Errors and lints](https://rustc-dev-guide.rust-lang.org/diagnostics.html)

use rustc_errors::{Diag, MultiSpan};
use rustc_hir::HirId;
use rustc_hir::attrs::LangItem;
use rustc_hir::def::DefKind;
use rustc_hir::def_id::{DefId, LocalDefId};
use rustc_middle::middle::codegen_fn_attrs::ferrocene::has_requires_validation_attribute;
use rustc_middle::ty::TyCtxt;
use rustc_span::{STDLIB_STABLE_CRATES, Span};
use tracing::debug;

use crate::ferrocene::post_mono::InstantiationSite;
use crate::ferrocene::{LintState, UNVALIDATED, UnvalidatedImplCause, Use, UseKind};

const ATTR_REQ: &str = "`#[ferrocene::requires_validation]`";
const ATTR_VAL: &str = "`#[ferrocene::prevalidated]`";

fn ident_span(tcx: TyCtxt<'_>, def_id: DefId) -> Span {
    tcx.def_ident_span(def_id).unwrap_or_else(|| tcx.def_span(def_id))
}

pub(super) fn error_prevalidated_without_default(
    tcx: TyCtxt<'_>,
    def_id: LocalDefId,
    attr_span: Span,
) {
    let def_id = def_id.to_def_id();
    let (article, descr) = tcx.article_and_description(def_id);
    let bov = body_or_value(tcx, def_id);
    tcx.dcx()
        .struct_span_err(
            attr_span,
            format!("{ATTR_VAL} cannot be applied to {article} {descr} without a default {bov}"),
        )
        .with_span_label(ident_span(tcx, def_id), format!("this {descr} has no {bov} to validate"))
        .with_help(format!(
            "use {ATTR_REQ} instead to require all implementations of this {descr} to be validated"
        ))
        .emit();
}

pub(super) fn error_requires_validation_wrong_item(
    tcx: TyCtxt<'_>,
    def_id: LocalDefId,
    attr_span: Span,
) {
    let def_id = def_id.to_def_id();
    let (article, descr) = tcx.article_and_description(def_id);
    tcx.dcx()
        .struct_span_err(attr_span, format!("{ATTR_REQ} cannot be applied to {article} {descr}"))
        .with_note(format!("{ATTR_REQ} can only be applied to associated trait items"))
        .emit();
}

pub(super) fn error_requires_validation_without_prevalidated(
    tcx: TyCtxt<'_>,
    def_id: LocalDefId,
    attr_span: Span,
) {
    let def_id = def_id.to_def_id();
    let (article, descr) = tcx.article_and_description(def_id);
    let bov = body_or_value(tcx, def_id);
    tcx.dcx()
        .struct_span_err(
            attr_span,
            format!(
                "{ATTR_REQ} on {article} {descr} with a default {bov} requires {ATTR_VAL} as well"
            ),
        )
        .with_span_label(
            ident_span(tcx, def_id),
            format!("this default {bov} is not validated, but implementations inherit it"),
        )
        .with_help(format!("add {ATTR_VAL} to validate the default {bov}"))
        .emit();
}

pub(super) fn error_const_no_fn_ptr(tcx: TyCtxt<'_>, def_id: LocalDefId, attr_span: Span) {
    tcx.dcx()
        .struct_span_err(
            attr_span,
            format!(
                "associated constants that are no function pointer cannot be marked {ATTR_REQ}"
            ),
        )
        .with_span_label(
            ident_span(tcx, def_id.to_def_id()),
            format!("this const is marked {ATTR_REQ}, but is no function pointer"),
        )
        .with_help(format!("remove {ATTR_REQ}"))
        .emit();
}

fn body_or_value(tcx: TyCtxt<'_>, def_id: DefId) -> &str {
    match tcx.def_kind(def_id) {
        DefKind::AssocConst { .. } => "value",
        DefKind::AssocFn => "body",
        _ => unreachable!("this must only be called on associated constants or functions"),
    }
}

pub(super) fn lint_impl_requires_validation(
    tcx: TyCtxt<'_>,
    implementation_id: LocalDefId,
    definition_id: DefId,
) {
    let implementation_hir_id = tcx.local_def_id_to_hir_id(implementation_id);
    let implementation_id = implementation_id.to_def_id();
    let implementation_span = ident_span(tcx, implementation_id);

    let descr = tcx.def_descr(implementation_id);
    tcx.emit_node_span_lint(
        UNVALIDATED,
        implementation_hir_id,
        implementation_span,
        rustc_errors::DiagDecorator(|diag| {
            let mut definition_span = MultiSpan::from_span(ident_span(tcx, definition_id));
            if let Some(annotation) = has_requires_validation_attribute(tcx, definition_id) {
                definition_span.push_span_label(annotation, "required to be validated here");
            }
            let definition_path = tcx.def_path_str(definition_id);

            diag.primary_message(format!(
                "unvalidated {descr} implements a trait item that requires validation",
            ))
            .span_label(implementation_span, "this implementation is unvalidated")
            .help("add `#[ferrocene::prevalidated]` to this implementation")
            .span_note(
                definition_span,
                format!("all implementations of `{definition_path}` must be validated"),
            );
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

        tcx.emit_node_span_lint(UNVALIDATED, lint_node, receiver_span, rustc_errors::DiagDecorator(|diag| {
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
            UseKind::Called(..) => "calls",
            // originally this said "type-erases" but that's unfamiliar jargon, and it's not clear
            // that it actually helps understanding.
            UseKind::TraitObjectCast(..) | UseKind::FnPtrCast(..) => "possibly calls",
            UseKind::ContainsFnPtr(..) => "uses",
        }
    }
}
