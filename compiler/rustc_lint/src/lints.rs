#![allow(rustc::diagnostic_outside_of_impl)]
#![allow(rustc::untranslatable_diagnostic)]
use std::num::NonZero;

use crate::errors::RequestedLevel;
use crate::fluent_generated as fluent;
use rustc_errors::{
    codes::*, Applicability, Diag, DiagMessage, DiagStyledString, EmissionGuarantee,
    LintDiagnostic, SubdiagMessageOp, Subdiagnostic, SuggestionStyle,
};
use rustc_hir::def_id::DefId;
use rustc_macros::{LintDiagnostic, Subdiagnostic};
use rustc_middle::ty::{
    inhabitedness::InhabitedPredicate, Clause, PolyExistentialTraitRef, Ty, TyCtxt,
};
use rustc_session::Session;
use rustc_span::{edition::Edition, sym, symbol::Ident, Span, Symbol};

use crate::{
    builtin::InitError, builtin::TypeAliasBounds, errors::OverruledAttributeSub, LateContext,
};

// array_into_iter.rs
#[derive(LintDiagnostic)]
#[diag(lint_array_into_iter)]
pub struct ArrayIntoIterDiag<'a> {
    pub target: &'a str,
    #[suggestion(lint_use_iter_suggestion, code = "iter", applicability = "machine-applicable")]
    pub suggestion: Span,
    #[subdiagnostic]
    pub sub: Option<ArrayIntoIterDiagSub>,
}

#[derive(Subdiagnostic)]
pub enum ArrayIntoIterDiagSub {
    #[suggestion(lint_remove_into_iter_suggestion, code = "", applicability = "maybe-incorrect")]
    RemoveIntoIter {
        #[primary_span]
        span: Span,
    },
    #[multipart_suggestion(
        lint_use_explicit_into_iter_suggestion,
        applicability = "maybe-incorrect"
    )]
    UseExplicitIntoIter {
        #[suggestion_part(code = "IntoIterator::into_iter(")]
        start_span: Span,
        #[suggestion_part(code = ")")]
        end_span: Span,
    },
}

// builtin.rs
#[derive(LintDiagnostic)]
#[diag(lint_builtin_while_true)]
pub struct BuiltinWhileTrue {
    #[suggestion(style = "short", code = "{replace}", applicability = "machine-applicable")]
    pub suggestion: Span,
    pub replace: String,
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_box_pointers)]
pub struct BuiltinBoxPointers<'a> {
    pub ty: Ty<'a>,
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_non_shorthand_field_patterns)]
pub struct BuiltinNonShorthandFieldPatterns {
    pub ident: Ident,
    #[suggestion(code = "{prefix}{ident}", applicability = "machine-applicable")]
    pub suggestion: Span,
    pub prefix: &'static str,
}

#[derive(LintDiagnostic)]
pub enum BuiltinUnsafe {
    #[diag(lint_builtin_allow_internal_unsafe)]
    AllowInternalUnsafe,
    #[diag(lint_builtin_unsafe_block)]
    UnsafeBlock,
    #[diag(lint_builtin_unsafe_trait)]
    UnsafeTrait,
    #[diag(lint_builtin_unsafe_impl)]
    UnsafeImpl,
    #[diag(lint_builtin_no_mangle_fn)]
    #[note(lint_builtin_overridden_symbol_name)]
    NoMangleFn,
    #[diag(lint_builtin_export_name_fn)]
    #[note(lint_builtin_overridden_symbol_name)]
    ExportNameFn,
    #[diag(lint_builtin_link_section_fn)]
    #[note(lint_builtin_overridden_symbol_section)]
    LinkSectionFn,
    #[diag(lint_builtin_no_mangle_static)]
    #[note(lint_builtin_overridden_symbol_name)]
    NoMangleStatic,
    #[diag(lint_builtin_export_name_static)]
    #[note(lint_builtin_overridden_symbol_name)]
    ExportNameStatic,
    #[diag(lint_builtin_link_section_static)]
    #[note(lint_builtin_overridden_symbol_section)]
    LinkSectionStatic,
    #[diag(lint_builtin_no_mangle_method)]
    #[note(lint_builtin_overridden_symbol_name)]
    NoMangleMethod,
    #[diag(lint_builtin_export_name_method)]
    #[note(lint_builtin_overridden_symbol_name)]
    ExportNameMethod,
    #[diag(lint_builtin_decl_unsafe_fn)]
    DeclUnsafeFn,
    #[diag(lint_builtin_decl_unsafe_method)]
    DeclUnsafeMethod,
    #[diag(lint_builtin_impl_unsafe_method)]
    ImplUnsafeMethod,
    #[diag(lint_builtin_global_asm)]
    #[note(lint_builtin_global_macro_unsafety)]
    GlobalAsm,
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_missing_doc)]
pub struct BuiltinMissingDoc<'a> {
    pub article: &'a str,
    pub desc: &'a str,
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_missing_copy_impl)]
pub struct BuiltinMissingCopyImpl;

pub struct BuiltinMissingDebugImpl<'a> {
    pub tcx: TyCtxt<'a>,
    pub def_id: DefId,
}

// Needed for def_path_str
impl<'a> LintDiagnostic<'a, ()> for BuiltinMissingDebugImpl<'_> {
    fn decorate_lint<'b>(self, diag: &'b mut rustc_errors::Diag<'a, ()>) {
        diag.arg("debug", self.tcx.def_path_str(self.def_id));
    }

    fn msg(&self) -> DiagMessage {
        fluent::lint_builtin_missing_debug_impl
    }
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_anonymous_params)]
pub struct BuiltinAnonymousParams<'a> {
    #[suggestion(code = "_: {ty_snip}")]
    pub suggestion: (Span, Applicability),
    pub ty_snip: &'a str,
}

// FIXME(davidtwco) translatable deprecated attr
#[derive(LintDiagnostic)]
#[diag(lint_builtin_deprecated_attr_link)]
pub struct BuiltinDeprecatedAttrLink<'a> {
    pub name: Symbol,
    pub reason: &'a str,
    pub link: &'a str,
    #[subdiagnostic]
    pub suggestion: BuiltinDeprecatedAttrLinkSuggestion<'a>,
}

#[derive(Subdiagnostic)]
pub enum BuiltinDeprecatedAttrLinkSuggestion<'a> {
    #[suggestion(lint_msg_suggestion, code = "", applicability = "machine-applicable")]
    Msg {
        #[primary_span]
        suggestion: Span,
        msg: &'a str,
    },
    #[suggestion(lint_default_suggestion, code = "", applicability = "machine-applicable")]
    Default {
        #[primary_span]
        suggestion: Span,
    },
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_deprecated_attr_used)]
pub struct BuiltinDeprecatedAttrUsed {
    pub name: String,
    #[suggestion(
        lint_builtin_deprecated_attr_default_suggestion,
        style = "short",
        code = "",
        applicability = "machine-applicable"
    )]
    pub suggestion: Span,
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_unused_doc_comment)]
pub struct BuiltinUnusedDocComment<'a> {
    pub kind: &'a str,
    #[label]
    pub label: Span,
    #[subdiagnostic]
    pub sub: BuiltinUnusedDocCommentSub,
}

#[derive(Subdiagnostic)]
pub enum BuiltinUnusedDocCommentSub {
    #[help(lint_plain_help)]
    PlainHelp,
    #[help(lint_block_help)]
    BlockHelp,
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_no_mangle_generic)]
pub struct BuiltinNoMangleGeneric {
    // Use of `#[no_mangle]` suggests FFI intent; correct
    // fix may be to monomorphize source by hand
    #[suggestion(style = "short", code = "", applicability = "maybe-incorrect")]
    pub suggestion: Span,
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_const_no_mangle)]
pub struct BuiltinConstNoMangle {
    #[suggestion(code = "pub static", applicability = "machine-applicable")]
    pub suggestion: Span,
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_mutable_transmutes)]
pub struct BuiltinMutablesTransmutes;

#[derive(LintDiagnostic)]
#[diag(lint_builtin_unstable_features)]
pub struct BuiltinUnstableFeatures;

// lint_ungated_async_fn_track_caller
pub struct BuiltinUngatedAsyncFnTrackCaller<'a> {
    pub label: Span,
    pub session: &'a Session,
}

impl<'a> LintDiagnostic<'a, ()> for BuiltinUngatedAsyncFnTrackCaller<'_> {
    fn decorate_lint<'b>(self, diag: &'b mut Diag<'a, ()>) {
        diag.span_label(self.label, fluent::lint_label);
        rustc_session::parse::add_feature_diagnostics(
            diag,
            self.session,
            sym::async_fn_track_caller,
        );
    }

    fn msg(&self) -> DiagMessage {
        fluent::lint_ungated_async_fn_track_caller
    }
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_unreachable_pub)]
pub struct BuiltinUnreachablePub<'a> {
    pub what: &'a str,
    #[suggestion(code = "pub(crate)")]
    pub suggestion: (Span, Applicability),
    #[help]
    pub help: Option<()>,
}

pub struct SuggestChangingAssocTypes<'a, 'b> {
    pub ty: &'a rustc_hir::Ty<'b>,
}

impl<'a, 'b> Subdiagnostic for SuggestChangingAssocTypes<'a, 'b> {
    fn add_to_diag_with<G: EmissionGuarantee, F: SubdiagMessageOp<G>>(
        self,
        diag: &mut Diag<'_, G>,
        _f: F,
    ) {
        // Access to associates types should use `<T as Bound>::Assoc`, which does not need a
        // bound. Let's see if this type does that.

        // We use a HIR visitor to walk the type.
        use rustc_hir::intravisit::{self, Visitor};
        struct WalkAssocTypes<'a, 'b, G: EmissionGuarantee> {
            err: &'a mut Diag<'b, G>,
        }
        impl<'a, 'b, G: EmissionGuarantee> Visitor<'_> for WalkAssocTypes<'a, 'b, G> {
            fn visit_qpath(
                &mut self,
                qpath: &rustc_hir::QPath<'_>,
                id: rustc_hir::HirId,
                span: Span,
            ) {
                if TypeAliasBounds::is_type_variable_assoc(qpath) {
                    self.err.span_help(span, fluent::lint_builtin_type_alias_bounds_help);
                }
                intravisit::walk_qpath(self, qpath, id)
            }
        }

        // Let's go for a walk!
        let mut visitor = WalkAssocTypes { err: diag };
        visitor.visit_ty(self.ty);
    }
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_type_alias_where_clause)]
pub struct BuiltinTypeAliasWhereClause<'a, 'b> {
    #[suggestion(code = "", applicability = "machine-applicable")]
    pub suggestion: Span,
    #[subdiagnostic]
    pub sub: Option<SuggestChangingAssocTypes<'a, 'b>>,
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_type_alias_generic_bounds)]
pub struct BuiltinTypeAliasGenericBounds<'a, 'b> {
    #[subdiagnostic]
    pub suggestion: BuiltinTypeAliasGenericBoundsSuggestion,
    #[subdiagnostic]
    pub sub: Option<SuggestChangingAssocTypes<'a, 'b>>,
}

pub struct BuiltinTypeAliasGenericBoundsSuggestion {
    pub suggestions: Vec<(Span, String)>,
}

impl Subdiagnostic for BuiltinTypeAliasGenericBoundsSuggestion {
    fn add_to_diag_with<G: EmissionGuarantee, F: SubdiagMessageOp<G>>(
        self,
        diag: &mut Diag<'_, G>,
        _f: F,
    ) {
        diag.multipart_suggestion(
            fluent::lint_suggestion,
            self.suggestions,
            Applicability::MachineApplicable,
        );
    }
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_trivial_bounds)]
pub struct BuiltinTrivialBounds<'a> {
    pub predicate_kind_name: &'a str,
    pub predicate: Clause<'a>,
}

#[derive(LintDiagnostic)]
pub enum BuiltinEllipsisInclusiveRangePatternsLint {
    #[diag(lint_builtin_ellipsis_inclusive_range_patterns)]
    Parenthesise {
        #[suggestion(code = "{replace}", applicability = "machine-applicable")]
        suggestion: Span,
        replace: String,
    },
    #[diag(lint_builtin_ellipsis_inclusive_range_patterns)]
    NonParenthesise {
        #[suggestion(style = "short", code = "..=", applicability = "machine-applicable")]
        suggestion: Span,
    },
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_keyword_idents)]
pub struct BuiltinKeywordIdents {
    pub kw: Ident,
    pub next: Edition,
    #[suggestion(code = "r#{kw}", applicability = "machine-applicable")]
    pub suggestion: Span,
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_explicit_outlives)]
pub struct BuiltinExplicitOutlives {
    pub count: usize,
    #[subdiagnostic]
    pub suggestion: BuiltinExplicitOutlivesSuggestion,
}

#[derive(Subdiagnostic)]
#[multipart_suggestion(lint_suggestion)]
pub struct BuiltinExplicitOutlivesSuggestion {
    #[suggestion_part(code = "")]
    pub spans: Vec<Span>,
    #[applicability]
    pub applicability: Applicability,
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_incomplete_features)]
pub struct BuiltinIncompleteFeatures {
    pub name: Symbol,
    #[subdiagnostic]
    pub note: Option<BuiltinFeatureIssueNote>,
    #[subdiagnostic]
    pub help: Option<BuiltinIncompleteFeaturesHelp>,
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_internal_features)]
#[note]
pub struct BuiltinInternalFeatures {
    pub name: Symbol,
}

#[derive(Subdiagnostic)]
#[help(lint_help)]
pub struct BuiltinIncompleteFeaturesHelp;

#[derive(Subdiagnostic)]
#[note(lint_note)]
pub struct BuiltinFeatureIssueNote {
    pub n: NonZero<u32>,
}

pub struct BuiltinUnpermittedTypeInit<'a> {
    pub msg: DiagMessage,
    pub ty: Ty<'a>,
    pub label: Span,
    pub sub: BuiltinUnpermittedTypeInitSub,
    pub tcx: TyCtxt<'a>,
}

impl<'a> LintDiagnostic<'a, ()> for BuiltinUnpermittedTypeInit<'_> {
    fn decorate_lint<'b>(self, diag: &'b mut Diag<'a, ()>) {
        diag.arg("ty", self.ty);
        diag.span_label(self.label, fluent::lint_builtin_unpermitted_type_init_label);
        if let InhabitedPredicate::True = self.ty.inhabited_predicate(self.tcx) {
            // Only suggest late `MaybeUninit::assume_init` initialization if the type is inhabited.
            diag.span_label(
                self.label,
                fluent::lint_builtin_unpermitted_type_init_label_suggestion,
            );
        }
        self.sub.add_to_diag(diag);
    }

    fn msg(&self) -> DiagMessage {
        self.msg.clone()
    }
}

// FIXME(davidtwco): make translatable
pub struct BuiltinUnpermittedTypeInitSub {
    pub err: InitError,
}

impl Subdiagnostic for BuiltinUnpermittedTypeInitSub {
    fn add_to_diag_with<G: EmissionGuarantee, F: SubdiagMessageOp<G>>(
        self,
        diag: &mut Diag<'_, G>,
        _f: F,
    ) {
        let mut err = self.err;
        loop {
            if let Some(span) = err.span {
                diag.span_note(span, err.message);
            } else {
                diag.note(err.message);
            }
            if let Some(e) = err.nested {
                err = *e;
            } else {
                break;
            }
        }
    }
}

#[derive(LintDiagnostic)]
pub enum BuiltinClashingExtern<'a> {
    #[diag(lint_builtin_clashing_extern_same_name)]
    SameName {
        this: Symbol,
        orig: Symbol,
        #[label(lint_previous_decl_label)]
        previous_decl_label: Span,
        #[label(lint_mismatch_label)]
        mismatch_label: Span,
        #[subdiagnostic]
        sub: BuiltinClashingExternSub<'a>,
    },
    #[diag(lint_builtin_clashing_extern_diff_name)]
    DiffName {
        this: Symbol,
        orig: Symbol,
        #[label(lint_previous_decl_label)]
        previous_decl_label: Span,
        #[label(lint_mismatch_label)]
        mismatch_label: Span,
        #[subdiagnostic]
        sub: BuiltinClashingExternSub<'a>,
    },
}

// FIXME(davidtwco): translatable expected/found
pub struct BuiltinClashingExternSub<'a> {
    pub tcx: TyCtxt<'a>,
    pub expected: Ty<'a>,
    pub found: Ty<'a>,
}

impl Subdiagnostic for BuiltinClashingExternSub<'_> {
    fn add_to_diag_with<G: EmissionGuarantee, F: SubdiagMessageOp<G>>(
        self,
        diag: &mut Diag<'_, G>,
        _f: F,
    ) {
        let mut expected_str = DiagStyledString::new();
        expected_str.push(self.expected.fn_sig(self.tcx).to_string(), false);
        let mut found_str = DiagStyledString::new();
        found_str.push(self.found.fn_sig(self.tcx).to_string(), true);
        diag.note_expected_found(&"", expected_str, &"", found_str);
    }
}

#[derive(LintDiagnostic)]
#[diag(lint_builtin_deref_nullptr)]
pub struct BuiltinDerefNullptr {
    #[label]
    pub label: Span,
}

// FIXME: migrate fluent::lint::builtin_asm_labels

#[derive(LintDiagnostic)]
pub enum BuiltinSpecialModuleNameUsed {
    #[diag(lint_builtin_special_module_name_used_lib)]
    #[note]
    #[help]
    Lib,
    #[diag(lint_builtin_special_module_name_used_main)]
    #[note]
    Main,
}

// deref_into_dyn_supertrait.rs
#[derive(LintDiagnostic)]
#[diag(lint_supertrait_as_deref_target)]
pub struct SupertraitAsDerefTarget<'a> {
    pub self_ty: Ty<'a>,
    pub supertrait_principal: PolyExistentialTraitRef<'a>,
    pub target_principal: PolyExistentialTraitRef<'a>,
    #[label]
    pub label: Span,
    #[subdiagnostic]
    pub label2: Option<SupertraitAsDerefTargetLabel>,
}

#[derive(Subdiagnostic)]
#[label(lint_label2)]
pub struct SupertraitAsDerefTargetLabel {
    #[primary_span]
    pub label: Span,
}

// enum_intrinsics_non_enums.rs
#[derive(LintDiagnostic)]
#[diag(lint_enum_intrinsics_mem_discriminant)]
pub struct EnumIntrinsicsMemDiscriminate<'a> {
    pub ty_param: Ty<'a>,
    #[note]
    pub note: Span,
}

#[derive(LintDiagnostic)]
#[diag(lint_enum_intrinsics_mem_variant)]
#[note]
pub struct EnumIntrinsicsMemVariant<'a> {
    pub ty_param: Ty<'a>,
}

// expect.rs
#[derive(LintDiagnostic)]
#[diag(lint_expectation)]
pub struct Expectation {
    #[subdiagnostic]
    pub rationale: Option<ExpectationNote>,
    #[note]
    pub note: Option<()>,
}

#[derive(Subdiagnostic)]
#[note(lint_rationale)]
pub struct ExpectationNote {
    pub rationale: Symbol,
}

// ptr_nulls.rs
#[derive(LintDiagnostic)]
pub enum PtrNullChecksDiag<'a> {
    #[diag(lint_ptr_null_checks_fn_ptr)]
    #[help(lint_help)]
    FnPtr {
        orig_ty: Ty<'a>,
        #[label]
        label: Span,
    },
    #[diag(lint_ptr_null_checks_ref)]
    Ref {
        orig_ty: Ty<'a>,
        #[label]
        label: Span,
    },
    #[diag(lint_ptr_null_checks_fn_ret)]
    FnRet { fn_name: Ident },
}

// for_loops_over_fallibles.rs
#[derive(LintDiagnostic)]
#[diag(lint_for_loops_over_fallibles)]
pub struct ForLoopsOverFalliblesDiag<'a> {
    pub article: &'static str,
    pub ty: &'static str,
    #[subdiagnostic]
    pub sub: ForLoopsOverFalliblesLoopSub<'a>,
    #[subdiagnostic]
    pub question_mark: Option<ForLoopsOverFalliblesQuestionMark>,
    #[subdiagnostic]
    pub suggestion: ForLoopsOverFalliblesSuggestion<'a>,
}

#[derive(Subdiagnostic)]
pub enum ForLoopsOverFalliblesLoopSub<'a> {
    #[suggestion(lint_remove_next, code = ".by_ref()", applicability = "maybe-incorrect")]
    RemoveNext {
        #[primary_span]
        suggestion: Span,
        recv_snip: String,
    },
    #[multipart_suggestion(lint_use_while_let, applicability = "maybe-incorrect")]
    UseWhileLet {
        #[suggestion_part(code = "while let {var}(")]
        start_span: Span,
        #[suggestion_part(code = ") = ")]
        end_span: Span,
        var: &'a str,
    },
}

#[derive(Subdiagnostic)]
#[suggestion(lint_use_question_mark, code = "?", applicability = "maybe-incorrect")]
pub struct ForLoopsOverFalliblesQuestionMark {
    #[primary_span]
    pub suggestion: Span,
}

#[derive(Subdiagnostic)]
#[multipart_suggestion(lint_suggestion, applicability = "maybe-incorrect")]
pub struct ForLoopsOverFalliblesSuggestion<'a> {
    pub var: &'a str,
    #[suggestion_part(code = "if let {var}(")]
    pub start_span: Span,
    #[suggestion_part(code = ") = ")]
    pub end_span: Span,
}

// drop_forget_useless.rs
#[derive(LintDiagnostic)]
#[diag(lint_dropping_references)]
#[note]
pub struct DropRefDiag<'a> {
    pub arg_ty: Ty<'a>,
    #[label]
    pub label: Span,
}

#[derive(LintDiagnostic)]
#[diag(lint_dropping_copy_types)]
#[note]
pub struct DropCopyDiag<'a> {
    pub arg_ty: Ty<'a>,
    #[label]
    pub label: Span,
}

#[derive(LintDiagnostic)]
#[diag(lint_forgetting_references)]
#[note]
pub struct ForgetRefDiag<'a> {
    pub arg_ty: Ty<'a>,
    #[label]
    pub label: Span,
}

#[derive(LintDiagnostic)]
#[diag(lint_forgetting_copy_types)]
#[note]
pub struct ForgetCopyDiag<'a> {
    pub arg_ty: Ty<'a>,
    #[label]
    pub label: Span,
}

#[derive(LintDiagnostic)]
#[diag(lint_undropped_manually_drops)]
pub struct UndroppedManuallyDropsDiag<'a> {
    pub arg_ty: Ty<'a>,
    #[label]
    pub label: Span,
    #[subdiagnostic]
    pub suggestion: UndroppedManuallyDropsSuggestion,
}

#[derive(Subdiagnostic)]
#[multipart_suggestion(lint_suggestion, applicability = "machine-applicable")]
pub struct UndroppedManuallyDropsSuggestion {
    #[suggestion_part(code = "std::mem::ManuallyDrop::into_inner(")]
    pub start_span: Span,
    #[suggestion_part(code = ")")]
    pub end_span: Span,
}

// invalid_from_utf8.rs
#[derive(LintDiagnostic)]
pub enum InvalidFromUtf8Diag {
    #[diag(lint_invalid_from_utf8_unchecked)]
    Unchecked {
        method: String,
        valid_up_to: usize,
        #[label]
        label: Span,
    },
    #[diag(lint_invalid_from_utf8_checked)]
    Checked {
        method: String,
        valid_up_to: usize,
        #[label]
        label: Span,
    },
}

// reference_casting.rs
#[derive(LintDiagnostic)]
pub enum InvalidReferenceCastingDiag<'tcx> {
    #[diag(lint_invalid_reference_casting_borrow_as_mut)]
    #[note(lint_invalid_reference_casting_note_book)]
    BorrowAsMut {
        #[label]
        orig_cast: Option<Span>,
        #[note(lint_invalid_reference_casting_note_ty_has_interior_mutability)]
        ty_has_interior_mutability: Option<()>,
    },
    #[diag(lint_invalid_reference_casting_assign_to_ref)]
    #[note(lint_invalid_reference_casting_note_book)]
    AssignToRef {
        #[label]
        orig_cast: Option<Span>,
        #[note(lint_invalid_reference_casting_note_ty_has_interior_mutability)]
        ty_has_interior_mutability: Option<()>,
    },
    #[diag(lint_invalid_reference_casting_bigger_layout)]
    #[note(lint_layout)]
    BiggerLayout {
        #[label]
        orig_cast: Option<Span>,
        #[label(lint_alloc)]
        alloc: Span,
        from_ty: Ty<'tcx>,
        from_size: u64,
        to_ty: Ty<'tcx>,
        to_size: u64,
    },
}

// hidden_unicode_codepoints.rs
#[derive(LintDiagnostic)]
#[diag(lint_hidden_unicode_codepoints)]
#[note]
pub struct HiddenUnicodeCodepointsDiag<'a> {
    pub label: &'a str,
    pub count: usize,
    #[label]
    pub span_label: Span,
    #[subdiagnostic]
    pub labels: Option<HiddenUnicodeCodepointsDiagLabels>,
    #[subdiagnostic]
    pub sub: HiddenUnicodeCodepointsDiagSub,
}

pub struct HiddenUnicodeCodepointsDiagLabels {
    pub spans: Vec<(char, Span)>,
}

impl Subdiagnostic for HiddenUnicodeCodepointsDiagLabels {
    fn add_to_diag_with<G: EmissionGuarantee, F: SubdiagMessageOp<G>>(
        self,
        diag: &mut Diag<'_, G>,
        _f: F,
    ) {
        for (c, span) in self.spans {
            diag.span_label(span, format!("{c:?}"));
        }
    }
}

pub enum HiddenUnicodeCodepointsDiagSub {
    Escape { spans: Vec<(char, Span)> },
    NoEscape { spans: Vec<(char, Span)> },
}

// Used because of multiple multipart_suggestion and note
impl Subdiagnostic for HiddenUnicodeCodepointsDiagSub {
    fn add_to_diag_with<G: EmissionGuarantee, F: SubdiagMessageOp<G>>(
        self,
        diag: &mut Diag<'_, G>,
        _f: F,
    ) {
        match self {
            HiddenUnicodeCodepointsDiagSub::Escape { spans } => {
                diag.multipart_suggestion_with_style(
                    fluent::lint_suggestion_remove,
                    spans.iter().map(|(_, span)| (*span, "".to_string())).collect(),
                    Applicability::MachineApplicable,
                    SuggestionStyle::HideCodeAlways,
                );
                diag.multipart_suggestion(
                    fluent::lint_suggestion_escape,
                    spans
                        .into_iter()
                        .map(|(c, span)| {
                            let c = format!("{c:?}");
                            (span, c[1..c.len() - 1].to_string())
                        })
                        .collect(),
                    Applicability::MachineApplicable,
                );
            }
            HiddenUnicodeCodepointsDiagSub::NoEscape { spans } => {
                // FIXME: in other suggestions we've reversed the inner spans of doc comments. We
                // should do the same here to provide the same good suggestions as we do for
                // literals above.
                diag.arg(
                    "escaped",
                    spans
                        .into_iter()
                        .map(|(c, _)| format!("{c:?}"))
                        .collect::<Vec<String>>()
                        .join(", "),
                );
                diag.note(fluent::lint_suggestion_remove);
                diag.note(fluent::lint_no_suggestion_note_escape);
            }
        }
    }
}

// map_unit_fn.rs
#[derive(LintDiagnostic)]
#[diag(lint_map_unit_fn)]
#[note]
pub struct MappingToUnit {
    #[label(lint_function_label)]
    pub function_label: Span,
    #[label(lint_argument_label)]
    pub argument_label: Span,
    #[label(lint_map_label)]
    pub map_label: Span,
    #[suggestion(style = "verbose", code = "{replace}", applicability = "maybe-incorrect")]
    pub suggestion: Span,
    pub replace: String,
}

// internal.rs
#[derive(LintDiagnostic)]
#[diag(lint_default_hash_types)]
#[note]
pub struct DefaultHashTypesDiag<'a> {
    pub preferred: &'a str,
    pub used: Symbol,
}

#[derive(LintDiagnostic)]
#[diag(lint_query_instability)]
#[note]
pub struct QueryInstability {
    pub query: Symbol,
}

#[derive(LintDiagnostic)]
#[diag(lint_span_use_eq_ctxt)]
pub struct SpanUseEqCtxtDiag;

#[derive(LintDiagnostic)]
#[diag(lint_tykind_kind)]
pub struct TykindKind {
    #[suggestion(code = "ty", applicability = "maybe-incorrect")]
    pub suggestion: Span,
}

#[derive(LintDiagnostic)]
#[diag(lint_tykind)]
#[help]
pub struct TykindDiag;

#[derive(LintDiagnostic)]
#[diag(lint_ty_qualified)]
pub struct TyQualified {
    pub ty: String,
    #[suggestion(code = "{ty}", applicability = "maybe-incorrect")]
    pub suggestion: Span,
}

#[derive(LintDiagnostic)]
#[diag(lint_lintpass_by_hand)]
#[help]
pub struct LintPassByHand;

#[derive(LintDiagnostic)]
#[diag(lint_non_existent_doc_keyword)]
#[help]
pub struct NonExistentDocKeyword {
    pub keyword: Symbol,
}

#[derive(LintDiagnostic)]
#[diag(lint_diag_out_of_impl)]
pub struct DiagOutOfImpl;

#[derive(LintDiagnostic)]
#[diag(lint_untranslatable_diag)]
pub struct UntranslatableDiag;

#[derive(LintDiagnostic)]
#[diag(lint_bad_opt_access)]
pub struct BadOptAccessDiag<'a> {
    pub msg: &'a str,
}

// let_underscore.rs
#[derive(LintDiagnostic)]
pub enum NonBindingLet {
    #[diag(lint_non_binding_let_on_sync_lock)]
    SyncLock {
        #[subdiagnostic]
        sub: NonBindingLetSub,
    },
    #[diag(lint_non_binding_let_on_drop_type)]
    DropType {
        #[subdiagnostic]
        sub: NonBindingLetSub,
    },
}

pub struct NonBindingLetSub {
    pub suggestion: Span,
    pub drop_fn_start_end: Option<(Span, Span)>,
    pub is_assign_desugar: bool,
}

impl Subdiagnostic for NonBindingLetSub {
    fn add_to_diag_with<G: EmissionGuarantee, F: SubdiagMessageOp<G>>(
        self,
        diag: &mut Diag<'_, G>,
        _f: F,
    ) {
        let can_suggest_binding = self.drop_fn_start_end.is_some() || !self.is_assign_desugar;

        if can_suggest_binding {
            let prefix = if self.is_assign_desugar { "let " } else { "" };
            diag.span_suggestion_verbose(
                self.suggestion,
                fluent::lint_non_binding_let_suggestion,
                format!("{prefix}_unused"),
                Applicability::MachineApplicable,
            );
        } else {
            diag.span_help(self.suggestion, fluent::lint_non_binding_let_suggestion);
        }
        if let Some(drop_fn_start_end) = self.drop_fn_start_end {
            diag.multipart_suggestion(
                fluent::lint_non_binding_let_multi_suggestion,
                vec![
                    (drop_fn_start_end.0, "drop(".to_string()),
                    (drop_fn_start_end.1, ")".to_string()),
                ],
                Applicability::MachineApplicable,
            );
        } else {
            diag.help(fluent::lint_non_binding_let_multi_drop_fn);
        }
    }
}

// levels.rs
#[derive(LintDiagnostic)]
#[diag(lint_overruled_attribute)]
pub struct OverruledAttributeLint<'a> {
    #[label]
    pub overruled: Span,
    pub lint_level: &'a str,
    pub lint_source: Symbol,
    #[subdiagnostic]
    pub sub: OverruledAttributeSub,
}

#[derive(LintDiagnostic)]
#[diag(lint_deprecated_lint_name)]
pub struct DeprecatedLintName<'a> {
    pub name: String,
    #[suggestion(code = "{replace}", applicability = "machine-applicable")]
    pub suggestion: Span,
    pub replace: &'a str,
}

#[derive(LintDiagnostic)]
#[diag(lint_deprecated_lint_name)]
#[help]
pub struct DeprecatedLintNameFromCommandLine<'a> {
    pub name: String,
    pub replace: &'a str,
    #[subdiagnostic]
    pub requested_level: RequestedLevel<'a>,
}

#[derive(LintDiagnostic)]
#[diag(lint_renamed_lint)]
pub struct RenamedLint<'a> {
    pub name: &'a str,
    #[subdiagnostic]
    pub suggestion: RenamedLintSuggestion<'a>,
}

#[derive(Subdiagnostic)]
pub enum RenamedLintSuggestion<'a> {
    #[suggestion(lint_suggestion, code = "{replace}", applicability = "machine-applicable")]
    WithSpan {
        #[primary_span]
        suggestion: Span,
        replace: &'a str,
    },
    #[help(lint_help)]
    WithoutSpan { replace: &'a str },
}

#[derive(LintDiagnostic)]
#[diag(lint_renamed_lint)]
pub struct RenamedLintFromCommandLine<'a> {
    pub name: &'a str,
    #[subdiagnostic]
    pub suggestion: RenamedLintSuggestion<'a>,
    #[subdiagnostic]
    pub requested_level: RequestedLevel<'a>,
}

#[derive(LintDiagnostic)]
#[diag(lint_removed_lint)]
pub struct RemovedLint<'a> {
    pub name: &'a str,
    pub reason: &'a str,
}

#[derive(LintDiagnostic)]
#[diag(lint_removed_lint)]
pub struct RemovedLintFromCommandLine<'a> {
    pub name: &'a str,
    pub reason: &'a str,
    #[subdiagnostic]
    pub requested_level: RequestedLevel<'a>,
}

#[derive(LintDiagnostic)]
#[diag(lint_unknown_lint)]
pub struct UnknownLint {
    pub name: String,
    #[subdiagnostic]
    pub suggestion: Option<UnknownLintSuggestion>,
}

#[derive(Subdiagnostic)]
pub enum UnknownLintSuggestion {
    #[suggestion(lint_suggestion, code = "{replace}", applicability = "maybe-incorrect")]
    WithSpan {
        #[primary_span]
        suggestion: Span,
        replace: Symbol,
        from_rustc: bool,
    },
    #[help(lint_help)]
    WithoutSpan { replace: Symbol, from_rustc: bool },
}

#[derive(LintDiagnostic)]
#[diag(lint_unknown_lint, code = E0602)]
pub struct UnknownLintFromCommandLine<'a> {
    pub name: String,
    #[subdiagnostic]
    pub suggestion: Option<UnknownLintSuggestion>,
    #[subdiagnostic]
    pub requested_level: RequestedLevel<'a>,
}

#[derive(LintDiagnostic)]
#[diag(lint_ignored_unless_crate_specified)]
pub struct IgnoredUnlessCrateSpecified<'a> {
    pub level: &'a str,
    pub name: Symbol,
}

// methods.rs
#[derive(LintDiagnostic)]
#[diag(lint_cstring_ptr)]
#[note]
#[help]
pub struct CStringPtr {
    #[label(lint_as_ptr_label)]
    pub as_ptr: Span,
    #[label(lint_unwrap_label)]
    pub unwrap: Span,
}

// multiple_supertrait_upcastable.rs
#[derive(LintDiagnostic)]
#[diag(lint_multiple_supertrait_upcastable)]
pub struct MultipleSupertraitUpcastable {
    pub ident: Ident,
}

// non_ascii_idents.rs
#[derive(LintDiagnostic)]
#[diag(lint_identifier_non_ascii_char)]
pub struct IdentifierNonAsciiChar;

#[derive(LintDiagnostic)]
#[diag(lint_identifier_uncommon_codepoints)]
#[note]
pub struct IdentifierUncommonCodepoints {
    pub codepoints: Vec<char>,
    pub codepoints_len: usize,
    pub identifier_type: &'static str,
}

#[derive(LintDiagnostic)]
#[diag(lint_confusable_identifier_pair)]
pub struct ConfusableIdentifierPair {
    pub existing_sym: Symbol,
    pub sym: Symbol,
    #[label(lint_other_use)]
    pub label: Span,
    #[label(lint_current_use)]
    pub main_label: Span,
}

#[derive(LintDiagnostic)]
#[diag(lint_mixed_script_confusables)]
#[note(lint_includes_note)]
#[note]
pub struct MixedScriptConfusables {
    pub set: String,
    pub includes: String,
}

// non_fmt_panic.rs
pub struct NonFmtPanicUnused {
    pub count: usize,
    pub suggestion: Option<Span>,
}

// Used because of two suggestions based on one Option<Span>
impl<'a> LintDiagnostic<'a, ()> for NonFmtPanicUnused {
    fn decorate_lint<'b>(self, diag: &'b mut Diag<'a, ()>) {
        diag.arg("count", self.count);
        diag.note(fluent::lint_note);
        if let Some(span) = self.suggestion {
            diag.span_suggestion(
                span.shrink_to_hi(),
                fluent::lint_add_args_suggestion,
                ", ...",
                Applicability::HasPlaceholders,
            );
            diag.span_suggestion(
                span.shrink_to_lo(),
                fluent::lint_add_fmt_suggestion,
                "\"{}\", ",
                Applicability::MachineApplicable,
            );
        }
    }

    fn msg(&self) -> DiagMessage {
        fluent::lint_non_fmt_panic_unused
    }
}

#[derive(LintDiagnostic)]
#[diag(lint_non_fmt_panic_braces)]
#[note]
pub struct NonFmtPanicBraces {
    pub count: usize,
    #[suggestion(code = "\"{{}}\", ", applicability = "machine-applicable")]
    pub suggestion: Option<Span>,
}

// nonstandard_style.rs
#[derive(LintDiagnostic)]
#[diag(lint_non_camel_case_type)]
pub struct NonCamelCaseType<'a> {
    pub sort: &'a str,
    pub name: &'a str,
    #[subdiagnostic]
    pub sub: NonCamelCaseTypeSub,
}

#[derive(Subdiagnostic)]
pub enum NonCamelCaseTypeSub {
    #[label(lint_label)]
    Label {
        #[primary_span]
        span: Span,
    },
    #[suggestion(lint_suggestion, code = "{replace}", applicability = "maybe-incorrect")]
    Suggestion {
        #[primary_span]
        span: Span,
        replace: String,
    },
}

#[derive(LintDiagnostic)]
#[diag(lint_non_snake_case)]
pub struct NonSnakeCaseDiag<'a> {
    pub sort: &'a str,
    pub name: &'a str,
    pub sc: String,
    #[subdiagnostic]
    pub sub: NonSnakeCaseDiagSub,
}

pub enum NonSnakeCaseDiagSub {
    Label { span: Span },
    Help,
    RenameOrConvertSuggestion { span: Span, suggestion: Ident },
    ConvertSuggestion { span: Span, suggestion: String },
    SuggestionAndNote { span: Span },
}

impl Subdiagnostic for NonSnakeCaseDiagSub {
    fn add_to_diag_with<G: EmissionGuarantee, F: SubdiagMessageOp<G>>(
        self,
        diag: &mut Diag<'_, G>,
        _f: F,
    ) {
        match self {
            NonSnakeCaseDiagSub::Label { span } => {
                diag.span_label(span, fluent::lint_label);
            }
            NonSnakeCaseDiagSub::Help => {
                diag.help(fluent::lint_help);
            }
            NonSnakeCaseDiagSub::ConvertSuggestion { span, suggestion } => {
                diag.span_suggestion(
                    span,
                    fluent::lint_convert_suggestion,
                    suggestion,
                    Applicability::MaybeIncorrect,
                );
            }
            NonSnakeCaseDiagSub::RenameOrConvertSuggestion { span, suggestion } => {
                diag.span_suggestion(
                    span,
                    fluent::lint_rename_or_convert_suggestion,
                    suggestion,
                    Applicability::MaybeIncorrect,
                );
            }
            NonSnakeCaseDiagSub::SuggestionAndNote { span } => {
                diag.note(fluent::lint_cannot_convert_note);
                diag.span_suggestion(
                    span,
                    fluent::lint_rename_suggestion,
                    "",
                    Applicability::MaybeIncorrect,
                );
            }
        }
    }
}

#[derive(LintDiagnostic)]
#[diag(lint_non_upper_case_global)]
pub struct NonUpperCaseGlobal<'a> {
    pub sort: &'a str,
    pub name: &'a str,
    #[subdiagnostic]
    pub sub: NonUpperCaseGlobalSub,
}

#[derive(Subdiagnostic)]
pub enum NonUpperCaseGlobalSub {
    #[label(lint_label)]
    Label {
        #[primary_span]
        span: Span,
    },
    #[suggestion(lint_suggestion, code = "{replace}", applicability = "maybe-incorrect")]
    Suggestion {
        #[primary_span]
        span: Span,
        replace: String,
    },
}

// noop_method_call.rs
#[derive(LintDiagnostic)]
#[diag(lint_noop_method_call)]
#[note]
pub struct NoopMethodCallDiag<'a> {
    pub method: Symbol,
    pub orig_ty: Ty<'a>,
    pub trait_: Symbol,
    #[suggestion(code = "", applicability = "machine-applicable")]
    pub label: Span,
    #[suggestion(
        lint_derive_suggestion,
        code = "#[derive(Clone)]\n",
        applicability = "maybe-incorrect"
    )]
    pub suggest_derive: Option<Span>,
}

#[derive(LintDiagnostic)]
#[diag(lint_suspicious_double_ref_deref)]
pub struct SuspiciousDoubleRefDerefDiag<'a> {
    pub ty: Ty<'a>,
}

#[derive(LintDiagnostic)]
#[diag(lint_suspicious_double_ref_clone)]
pub struct SuspiciousDoubleRefCloneDiag<'a> {
    pub ty: Ty<'a>,
}

// non_local_defs.rs
#[derive(LintDiagnostic)]
pub enum NonLocalDefinitionsDiag {
    #[diag(lint_non_local_definitions_impl)]
    #[help]
    #[note(lint_non_local)]
    #[note(lint_exception)]
    #[note(lint_non_local_definitions_deprecation)]
    Impl {
        depth: u32,
        body_kind_descr: &'static str,
        body_name: String,
        #[subdiagnostic]
        cargo_update: Option<NonLocalDefinitionsCargoUpdateNote>,
        #[suggestion(lint_const_anon, code = "_", applicability = "machine-applicable")]
        const_anon: Option<Span>,
    },
    #[diag(lint_non_local_definitions_macro_rules)]
    #[help]
    #[note(lint_non_local)]
    #[note(lint_exception)]
    #[note(lint_non_local_definitions_deprecation)]
    MacroRules {
        depth: u32,
        body_kind_descr: &'static str,
        body_name: String,
        #[subdiagnostic]
        cargo_update: Option<NonLocalDefinitionsCargoUpdateNote>,
    },
}

#[derive(Subdiagnostic)]
#[note(lint_non_local_definitions_cargo_update)]
pub struct NonLocalDefinitionsCargoUpdateNote {
    pub macro_kind: &'static str,
    pub macro_name: Symbol,
    pub crate_name: Symbol,
}

// pass_by_value.rs
#[derive(LintDiagnostic)]
#[diag(lint_pass_by_value)]
pub struct PassByValueDiag {
    pub ty: String,
    #[suggestion(code = "{ty}", applicability = "maybe-incorrect")]
    pub suggestion: Span,
}

// redundant_semicolon.rs
#[derive(LintDiagnostic)]
#[diag(lint_redundant_semicolons)]
pub struct RedundantSemicolonsDiag {
    pub multiple: bool,
    #[suggestion(code = "", applicability = "maybe-incorrect")]
    pub suggestion: Span,
}

// traits.rs
pub struct DropTraitConstraintsDiag<'a> {
    pub predicate: Clause<'a>,
    pub tcx: TyCtxt<'a>,
    pub def_id: DefId,
}

// Needed for def_path_str
impl<'a> LintDiagnostic<'a, ()> for DropTraitConstraintsDiag<'_> {
    fn decorate_lint<'b>(self, diag: &'b mut Diag<'a, ()>) {
        diag.arg("predicate", self.predicate);
        diag.arg("needs_drop", self.tcx.def_path_str(self.def_id));
    }

    fn msg(&self) -> DiagMessage {
        fluent::lint_drop_trait_constraints
    }
}

pub struct DropGlue<'a> {
    pub tcx: TyCtxt<'a>,
    pub def_id: DefId,
}

// Needed for def_path_str
impl<'a> LintDiagnostic<'a, ()> for DropGlue<'_> {
    fn decorate_lint<'b>(self, diag: &'b mut Diag<'a, ()>) {
        diag.arg("needs_drop", self.tcx.def_path_str(self.def_id));
    }

    fn msg(&self) -> DiagMessage {
        fluent::lint_drop_glue
    }
}

// types.rs
#[derive(LintDiagnostic)]
#[diag(lint_range_endpoint_out_of_range)]
pub struct RangeEndpointOutOfRange<'a> {
    pub ty: &'a str,
    #[subdiagnostic]
    pub sub: UseInclusiveRange<'a>,
}

#[derive(Subdiagnostic)]
pub enum UseInclusiveRange<'a> {
    #[suggestion(
        lint_range_use_inclusive_range,
        code = "{start}..={literal}{suffix}",
        applicability = "machine-applicable"
    )]
    WithoutParen {
        #[primary_span]
        sugg: Span,
        start: String,
        literal: u128,
        suffix: &'a str,
    },
    #[multipart_suggestion(lint_range_use_inclusive_range, applicability = "machine-applicable")]
    WithParen {
        #[suggestion_part(code = "=")]
        eq_sugg: Span,
        #[suggestion_part(code = "{literal}{suffix}")]
        lit_sugg: Span,
        literal: u128,
        suffix: &'a str,
    },
}

#[derive(LintDiagnostic)]
#[diag(lint_overflowing_bin_hex)]
pub struct OverflowingBinHex<'a> {
    pub ty: &'a str,
    pub lit: String,
    pub dec: u128,
    pub actually: String,
    #[subdiagnostic]
    pub sign: OverflowingBinHexSign,
    #[subdiagnostic]
    pub sub: Option<OverflowingBinHexSub<'a>>,
    #[subdiagnostic]
    pub sign_bit_sub: Option<OverflowingBinHexSignBitSub<'a>>,
}

pub enum OverflowingBinHexSign {
    Positive,
    Negative,
}

impl Subdiagnostic for OverflowingBinHexSign {
    fn add_to_diag_with<G: EmissionGuarantee, F: SubdiagMessageOp<G>>(
        self,
        diag: &mut Diag<'_, G>,
        _f: F,
    ) {
        match self {
            OverflowingBinHexSign::Positive => {
                diag.note(fluent::lint_positive_note);
            }
            OverflowingBinHexSign::Negative => {
                diag.note(fluent::lint_negative_note);
                diag.note(fluent::lint_negative_becomes_note);
            }
        }
    }
}

#[derive(Subdiagnostic)]
pub enum OverflowingBinHexSub<'a> {
    #[suggestion(
        lint_suggestion,
        code = "{sans_suffix}{suggestion_ty}",
        applicability = "machine-applicable"
    )]
    Suggestion {
        #[primary_span]
        span: Span,
        suggestion_ty: &'a str,
        sans_suffix: &'a str,
    },
    #[help(lint_help)]
    Help { suggestion_ty: &'a str },
}

#[derive(Subdiagnostic)]
#[suggestion(
    lint_sign_bit_suggestion,
    code = "{lit_no_suffix}{uint_ty} as {int_ty}",
    applicability = "maybe-incorrect"
)]
pub struct OverflowingBinHexSignBitSub<'a> {
    #[primary_span]
    pub span: Span,
    pub lit_no_suffix: &'a str,
    pub negative_val: String,
    pub uint_ty: &'a str,
    pub int_ty: &'a str,
}

#[derive(LintDiagnostic)]
#[diag(lint_overflowing_int)]
#[note]
pub struct OverflowingInt<'a> {
    pub ty: &'a str,
    pub lit: String,
    pub min: i128,
    pub max: u128,
    #[subdiagnostic]
    pub help: Option<OverflowingIntHelp<'a>>,
}

#[derive(Subdiagnostic)]
#[help(lint_help)]
pub struct OverflowingIntHelp<'a> {
    pub suggestion_ty: &'a str,
}

#[derive(LintDiagnostic)]
#[diag(lint_only_cast_u8_to_char)]
pub struct OnlyCastu8ToChar {
    #[suggestion(code = "'\\u{{{literal:X}}}'", applicability = "machine-applicable")]
    pub span: Span,
    pub literal: u128,
}

#[derive(LintDiagnostic)]
#[diag(lint_overflowing_uint)]
#[note]
pub struct OverflowingUInt<'a> {
    pub ty: &'a str,
    pub lit: String,
    pub min: u128,
    pub max: u128,
}

#[derive(LintDiagnostic)]
#[diag(lint_overflowing_literal)]
#[note]
pub struct OverflowingLiteral<'a> {
    pub ty: &'a str,
    pub lit: String,
}

#[derive(LintDiagnostic)]
#[diag(lint_unused_comparisons)]
pub struct UnusedComparisons;

#[derive(LintDiagnostic)]
pub enum InvalidNanComparisons {
    #[diag(lint_invalid_nan_comparisons_eq_ne)]
    EqNe {
        #[subdiagnostic]
        suggestion: Option<InvalidNanComparisonsSuggestion>,
    },
    #[diag(lint_invalid_nan_comparisons_lt_le_gt_ge)]
    LtLeGtGe,
}

#[derive(Subdiagnostic)]
pub enum InvalidNanComparisonsSuggestion {
    #[multipart_suggestion(
        lint_suggestion,
        style = "verbose",
        applicability = "machine-applicable"
    )]
    Spanful {
        #[suggestion_part(code = "!")]
        neg: Option<Span>,
        #[suggestion_part(code = ".is_nan()")]
        float: Span,
        #[suggestion_part(code = "")]
        nan_plus_binop: Span,
    },
    #[help(lint_suggestion)]
    Spanless,
}

#[derive(LintDiagnostic)]
pub enum AmbiguousWidePointerComparisons<'a> {
    #[diag(lint_ambiguous_wide_pointer_comparisons)]
    Spanful {
        #[subdiagnostic]
        addr_suggestion: AmbiguousWidePointerComparisonsAddrSuggestion<'a>,
        #[subdiagnostic]
        addr_metadata_suggestion: Option<AmbiguousWidePointerComparisonsAddrMetadataSuggestion<'a>>,
    },
    #[diag(lint_ambiguous_wide_pointer_comparisons)]
    #[help(lint_addr_metadata_suggestion)]
    #[help(lint_addr_suggestion)]
    Spanless,
}

#[derive(Subdiagnostic)]
#[multipart_suggestion(
    lint_addr_metadata_suggestion,
    style = "verbose",
    // FIXME(#53934): make machine-applicable again
    applicability = "maybe-incorrect"
)]
pub struct AmbiguousWidePointerComparisonsAddrMetadataSuggestion<'a> {
    pub ne: &'a str,
    pub deref_left: &'a str,
    pub deref_right: &'a str,
    #[suggestion_part(code = "{ne}std::ptr::eq({deref_left}")]
    pub left: Span,
    #[suggestion_part(code = ", {deref_right}")]
    pub middle: Span,
    #[suggestion_part(code = ")")]
    pub right: Span,
}

#[derive(Subdiagnostic)]
pub enum AmbiguousWidePointerComparisonsAddrSuggestion<'a> {
    #[multipart_suggestion(
        lint_addr_suggestion,
        style = "verbose",
        // FIXME(#53934): make machine-applicable again
        applicability = "maybe-incorrect"
    )]
    AddrEq {
        ne: &'a str,
        deref_left: &'a str,
        deref_right: &'a str,
        #[suggestion_part(code = "{ne}std::ptr::addr_eq({deref_left}")]
        left: Span,
        #[suggestion_part(code = ", {deref_right}")]
        middle: Span,
        #[suggestion_part(code = ")")]
        right: Span,
    },
    #[multipart_suggestion(
        lint_addr_suggestion,
        style = "verbose",
        // FIXME(#53934): make machine-applicable again
        applicability = "maybe-incorrect"
    )]
    Cast {
        deref_left: &'a str,
        deref_right: &'a str,
        #[suggestion_part(code = "{deref_left}")]
        left_before: Option<Span>,
        #[suggestion_part(code = " as *const ()")]
        left: Span,
        #[suggestion_part(code = "{deref_right}")]
        right_before: Option<Span>,
        #[suggestion_part(code = " as *const ()")]
        right: Span,
    },
}

pub struct ImproperCTypes<'a> {
    pub ty: Ty<'a>,
    pub desc: &'a str,
    pub label: Span,
    pub help: Option<DiagMessage>,
    pub note: DiagMessage,
    pub span_note: Option<Span>,
}

// Used because of the complexity of Option<DiagMessage>, DiagMessage, and Option<Span>
impl<'a> LintDiagnostic<'a, ()> for ImproperCTypes<'_> {
    fn decorate_lint<'b>(self, diag: &'b mut Diag<'a, ()>) {
        diag.arg("ty", self.ty);
        diag.arg("desc", self.desc);
        diag.span_label(self.label, fluent::lint_label);
        if let Some(help) = self.help {
            diag.help(help);
        }
        diag.note(self.note);
        if let Some(note) = self.span_note {
            diag.span_note(note, fluent::lint_note);
        }
    }

    fn msg(&self) -> DiagMessage {
        fluent::lint_improper_ctypes
    }
}

#[derive(LintDiagnostic)]
#[diag(lint_variant_size_differences)]
pub struct VariantSizeDifferencesDiag {
    pub largest: u64,
}

#[derive(LintDiagnostic)]
#[diag(lint_atomic_ordering_load)]
#[help]
pub struct AtomicOrderingLoad;

#[derive(LintDiagnostic)]
#[diag(lint_atomic_ordering_store)]
#[help]
pub struct AtomicOrderingStore;

#[derive(LintDiagnostic)]
#[diag(lint_atomic_ordering_fence)]
#[help]
pub struct AtomicOrderingFence;

#[derive(LintDiagnostic)]
#[diag(lint_atomic_ordering_invalid)]
#[help]
pub struct InvalidAtomicOrderingDiag {
    pub method: Symbol,
    #[label]
    pub fail_order_arg_span: Span,
}

// unused.rs
#[derive(LintDiagnostic)]
#[diag(lint_unused_op)]
pub struct UnusedOp<'a> {
    pub op: &'a str,
    #[label]
    pub label: Span,
    #[subdiagnostic]
    pub suggestion: UnusedOpSuggestion,
}

#[derive(Subdiagnostic)]
pub enum UnusedOpSuggestion {
    #[suggestion(
        lint_suggestion,
        style = "verbose",
        code = "let _ = ",
        applicability = "maybe-incorrect"
    )]
    NormalExpr {
        #[primary_span]
        span: Span,
    },
    #[multipart_suggestion(lint_suggestion, style = "verbose", applicability = "maybe-incorrect")]
    BlockTailExpr {
        #[suggestion_part(code = "let _ = ")]
        before_span: Span,
        #[suggestion_part(code = ";")]
        after_span: Span,
    },
}

#[derive(LintDiagnostic)]
#[diag(lint_unused_result)]
pub struct UnusedResult<'a> {
    pub ty: Ty<'a>,
}

// FIXME(davidtwco): this isn't properly translatable because of the
// pre/post strings
#[derive(LintDiagnostic)]
#[diag(lint_unused_closure)]
#[note]
pub struct UnusedClosure<'a> {
    pub count: usize,
    pub pre: &'a str,
    pub post: &'a str,
}

// FIXME(davidtwco): this isn't properly translatable because of the
// pre/post strings
#[derive(LintDiagnostic)]
#[diag(lint_unused_coroutine)]
#[note]
pub struct UnusedCoroutine<'a> {
    pub count: usize,
    pub pre: &'a str,
    pub post: &'a str,
}

// FIXME(davidtwco): this isn't properly translatable because of the pre/post
// strings
pub struct UnusedDef<'a, 'b> {
    pub pre: &'a str,
    pub post: &'a str,
    pub cx: &'a LateContext<'b>,
    pub def_id: DefId,
    pub note: Option<Symbol>,
    pub suggestion: Option<UnusedDefSuggestion>,
}

#[derive(Subdiagnostic)]

pub enum UnusedDefSuggestion {
    #[suggestion(
        lint_suggestion,
        style = "verbose",
        code = "let _ = ",
        applicability = "maybe-incorrect"
    )]
    NormalExpr {
        #[primary_span]
        span: Span,
    },
    #[multipart_suggestion(lint_suggestion, style = "verbose", applicability = "maybe-incorrect")]
    BlockTailExpr {
        #[suggestion_part(code = "let _ = ")]
        before_span: Span,
        #[suggestion_part(code = ";")]
        after_span: Span,
    },
}

// Needed because of def_path_str
impl<'a> LintDiagnostic<'a, ()> for UnusedDef<'_, '_> {
    fn decorate_lint<'b>(self, diag: &'b mut Diag<'a, ()>) {
        diag.arg("pre", self.pre);
        diag.arg("post", self.post);
        diag.arg("def", self.cx.tcx.def_path_str(self.def_id));
        // check for #[must_use = "..."]
        if let Some(note) = self.note {
            diag.note(note.to_string());
        }
        if let Some(sugg) = self.suggestion {
            diag.subdiagnostic(diag.dcx, sugg);
        }
    }

    fn msg(&self) -> DiagMessage {
        fluent::lint_unused_def
    }
}

#[derive(LintDiagnostic)]
#[diag(lint_path_statement_drop)]
pub struct PathStatementDrop {
    #[subdiagnostic]
    pub sub: PathStatementDropSub,
}

#[derive(Subdiagnostic)]
pub enum PathStatementDropSub {
    #[suggestion(lint_suggestion, code = "drop({snippet});", applicability = "machine-applicable")]
    Suggestion {
        #[primary_span]
        span: Span,
        snippet: String,
    },
    #[help(lint_help)]
    Help {
        #[primary_span]
        span: Span,
    },
}

#[derive(LintDiagnostic)]
#[diag(lint_path_statement_no_effect)]
pub struct PathStatementNoEffect;

#[derive(LintDiagnostic)]
#[diag(lint_unused_delim)]
pub struct UnusedDelim<'a> {
    pub delim: &'static str,
    pub item: &'a str,
    #[subdiagnostic]
    pub suggestion: Option<UnusedDelimSuggestion>,
}

#[derive(Subdiagnostic)]
#[multipart_suggestion(lint_suggestion, applicability = "machine-applicable")]
pub struct UnusedDelimSuggestion {
    #[suggestion_part(code = "{start_replace}")]
    pub start_span: Span,
    pub start_replace: &'static str,
    #[suggestion_part(code = "{end_replace}")]
    pub end_span: Span,
    pub end_replace: &'static str,
}

#[derive(LintDiagnostic)]
#[diag(lint_unused_import_braces)]
pub struct UnusedImportBracesDiag {
    pub node: Symbol,
}

#[derive(LintDiagnostic)]
#[diag(lint_unused_allocation)]
pub struct UnusedAllocationDiag;

#[derive(LintDiagnostic)]
#[diag(lint_unused_allocation_mut)]
pub struct UnusedAllocationMutDiag;

pub struct AsyncFnInTraitDiag {
    pub sugg: Option<Vec<(Span, String)>>,
}

impl<'a> LintDiagnostic<'a, ()> for AsyncFnInTraitDiag {
    fn decorate_lint<'b>(self, diag: &'b mut Diag<'a, ()>) {
        diag.note(fluent::lint_note);
        if let Some(sugg) = self.sugg {
            diag.multipart_suggestion(fluent::lint_suggestion, sugg, Applicability::MaybeIncorrect);
        }
    }

    fn msg(&self) -> DiagMessage {
        fluent::lint_async_fn_in_trait
    }
}

#[derive(LintDiagnostic)]
#[diag(lint_unit_bindings)]
pub struct UnitBindingsDiag {
    #[label]
    pub label: Span,
}
