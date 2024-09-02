//! Diagnostics rendering and fixits.
//!
//! Most of the diagnostics originate from the dark depth of the compiler, and
//! are originally expressed in term of IR. When we emit the diagnostic, we are
//! usually not in the position to decide how to best "render" it in terms of
//! user-authored source code. We are especially not in the position to offer
//! fixits, as the compiler completely lacks the infrastructure to edit the
//! source code.
//!
//! Instead, we "bubble up" raw, structured diagnostics until the `hir` crate,
//! where we "cook" them so that each diagnostic is formulated in terms of `hir`
//! types. Well, at least that's the aspiration, the "cooking" is somewhat
//! ad-hoc at the moment. Anyways, we get a bunch of ide-friendly diagnostic
//! structs from hir, and we want to render them to unified serializable
//! representation (span, level, message) here. If we can, we also provide
//! fixits. By the way, that's why we want to keep diagnostics structured
//! internally -- so that we have all the info to make fixes.
//!
//! We have one "handler" module per diagnostic code. Such a module contains
//! rendering, optional fixes and tests. It's OK if some low-level compiler
//! functionality ends up being tested via a diagnostic.
//!
//! There are also a couple of ad-hoc diagnostics implemented directly here, we
//! don't yet have a great pattern for how to do them properly.

mod handlers {
    pub(crate) mod await_outside_of_async;
    pub(crate) mod break_outside_of_loop;
    pub(crate) mod expected_function;
    pub(crate) mod inactive_code;
    pub(crate) mod incoherent_impl;
    pub(crate) mod incorrect_case;
    pub(crate) mod invalid_derive_target;
    pub(crate) mod macro_error;
    pub(crate) mod malformed_derive;
    pub(crate) mod mismatched_arg_count;
    pub(crate) mod missing_fields;
    pub(crate) mod missing_match_arms;
    pub(crate) mod missing_unsafe;
    pub(crate) mod moved_out_of_ref;
    pub(crate) mod mutability_errors;
    pub(crate) mod no_such_field;
    pub(crate) mod non_exhaustive_let;
    pub(crate) mod private_assoc_item;
    pub(crate) mod private_field;
    pub(crate) mod remove_trailing_return;
    pub(crate) mod remove_unnecessary_else;
    pub(crate) mod replace_filter_map_next_with_find_map;
    pub(crate) mod trait_impl_incorrect_safety;
    pub(crate) mod trait_impl_missing_assoc_item;
    pub(crate) mod trait_impl_orphan;
    pub(crate) mod trait_impl_redundant_assoc_item;
    pub(crate) mod type_mismatch;
    pub(crate) mod typed_hole;
    pub(crate) mod undeclared_label;
    pub(crate) mod unimplemented_builtin_macro;
    pub(crate) mod unreachable_label;
    pub(crate) mod unresolved_assoc_item;
    pub(crate) mod unresolved_extern_crate;
    pub(crate) mod unresolved_field;
    pub(crate) mod unresolved_ident;
    pub(crate) mod unresolved_import;
    pub(crate) mod unresolved_macro_call;
    pub(crate) mod unresolved_method;
    pub(crate) mod unresolved_module;
    pub(crate) mod unused_variables;

    // The handlers below are unusual, the implement the diagnostics as well.
    pub(crate) mod field_shorthand;
    pub(crate) mod json_is_not_rust;
    pub(crate) mod unlinked_file;
    pub(crate) mod useless_braces;
}

#[cfg(test)]
mod tests;

use std::sync::LazyLock;

use hir::{diagnostics::AnyDiagnostic, InFile, Semantics};
use ide_db::{
    assists::{Assist, AssistId, AssistKind, AssistResolveStrategy},
    base_db::SourceDatabase,
    generated::lints::{LintGroup, CLIPPY_LINT_GROUPS, DEFAULT_LINT_GROUPS},
    imports::insert_use::InsertUseConfig,
    label::Label,
    source_change::SourceChange,
    syntax_helpers::node_ext::parse_tt_as_comma_sep_paths,
    EditionedFileId, FileId, FileRange, FxHashMap, FxHashSet, RootDatabase, SnippetCap,
};
use stdx::never;
use syntax::{
    ast::{self, AstNode},
    AstPtr, Edition, SyntaxNode, SyntaxNodePtr, TextRange,
};

// FIXME: Make this an enum
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum DiagnosticCode {
    RustcHardError(&'static str),
    SyntaxError,
    RustcLint(&'static str),
    Clippy(&'static str),
    Ra(&'static str, Severity),
}

impl DiagnosticCode {
    pub fn url(&self) -> String {
        match self {
            DiagnosticCode::RustcHardError(e) => {
                format!("https://doc.rust-lang.org/stable/error_codes/{e}.html")
            }
            DiagnosticCode::SyntaxError => {
                String::from("https://doc.rust-lang.org/stable/reference/")
            }
            DiagnosticCode::RustcLint(e) => {
                format!("https://doc.rust-lang.org/rustc/?search={e}")
            }
            DiagnosticCode::Clippy(e) => {
                format!("https://rust-lang.github.io/rust-clippy/master/#/{e}")
            }
            DiagnosticCode::Ra(e, _) => {
                format!("https://rust-analyzer.github.io/manual.html#{e}")
            }
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            DiagnosticCode::RustcHardError(r)
            | DiagnosticCode::RustcLint(r)
            | DiagnosticCode::Clippy(r)
            | DiagnosticCode::Ra(r, _) => r,
            DiagnosticCode::SyntaxError => "syntax-error",
        }
    }
}

#[derive(Debug)]
pub struct Diagnostic {
    pub code: DiagnosticCode,
    pub message: String,
    pub range: FileRange,
    pub severity: Severity,
    pub unused: bool,
    pub experimental: bool,
    pub fixes: Option<Vec<Assist>>,
    // The node that will be affected by `#[allow]` and similar attributes.
    pub main_node: Option<InFile<SyntaxNodePtr>>,
}

impl Diagnostic {
    fn new(
        code: DiagnosticCode,
        message: impl Into<String>,
        range: impl Into<FileRange>,
    ) -> Diagnostic {
        let message = message.into();
        Diagnostic {
            code,
            message,
            range: range.into(),
            severity: match code {
                DiagnosticCode::RustcHardError(_) | DiagnosticCode::SyntaxError => Severity::Error,
                // FIXME: Rustc lints are not always warning, but the ones that are currently implemented are all warnings.
                DiagnosticCode::RustcLint(_) => Severity::Warning,
                // FIXME: We can make this configurable, and if the user uses `cargo clippy` on flycheck, we can
                // make it normal warning.
                DiagnosticCode::Clippy(_) => Severity::WeakWarning,
                DiagnosticCode::Ra(_, s) => s,
            },
            unused: false,
            experimental: false,
            fixes: None,
            main_node: None,
        }
    }

    fn new_with_syntax_node_ptr(
        ctx: &DiagnosticsContext<'_>,
        code: DiagnosticCode,
        message: impl Into<String>,
        node: InFile<SyntaxNodePtr>,
    ) -> Diagnostic {
        Diagnostic::new(code, message, ctx.sema.diagnostics_display_range(node))
            .with_main_node(node)
    }

    fn experimental(mut self) -> Diagnostic {
        self.experimental = true;
        self
    }

    fn with_main_node(mut self, main_node: InFile<SyntaxNodePtr>) -> Diagnostic {
        self.main_node = Some(main_node);
        self
    }

    fn with_fixes(mut self, fixes: Option<Vec<Assist>>) -> Diagnostic {
        self.fixes = fixes;
        self
    }

    fn with_unused(mut self, unused: bool) -> Diagnostic {
        self.unused = unused;
        self
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Severity {
    Error,
    Warning,
    WeakWarning,
    Allow,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprFillDefaultMode {
    Todo,
    Default,
}
impl Default for ExprFillDefaultMode {
    fn default() -> Self {
        Self::Todo
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticsConfig {
    /// Whether native diagnostics are enabled.
    pub enabled: bool,
    pub proc_macros_enabled: bool,
    pub proc_attr_macros_enabled: bool,
    pub disable_experimental: bool,
    pub disabled: FxHashSet<String>,
    pub expr_fill_default: ExprFillDefaultMode,
    pub style_lints: bool,
    // FIXME: We may want to include a whole `AssistConfig` here
    pub snippet_cap: Option<SnippetCap>,
    pub insert_use: InsertUseConfig,
    pub prefer_no_std: bool,
    pub prefer_prelude: bool,
    pub prefer_absolute: bool,
    pub term_search_fuel: u64,
    pub term_search_borrowck: bool,
}

impl DiagnosticsConfig {
    pub fn test_sample() -> Self {
        use hir::PrefixKind;
        use ide_db::imports::insert_use::ImportGranularity;

        Self {
            enabled: true,
            proc_macros_enabled: Default::default(),
            proc_attr_macros_enabled: Default::default(),
            disable_experimental: Default::default(),
            disabled: Default::default(),
            expr_fill_default: Default::default(),
            style_lints: true,
            snippet_cap: SnippetCap::new(true),
            insert_use: InsertUseConfig {
                granularity: ImportGranularity::Preserve,
                enforce_granularity: false,
                prefix_kind: PrefixKind::Plain,
                group: false,
                skip_glob_imports: false,
            },
            prefer_no_std: false,
            prefer_prelude: true,
            prefer_absolute: false,
            term_search_fuel: 400,
            term_search_borrowck: true,
        }
    }
}

struct DiagnosticsContext<'a> {
    config: &'a DiagnosticsConfig,
    sema: Semantics<'a, RootDatabase>,
    resolve: &'a AssistResolveStrategy,
    edition: Edition,
}

impl DiagnosticsContext<'_> {
    fn resolve_precise_location(
        &self,
        node: &InFile<SyntaxNodePtr>,
        precise_location: Option<TextRange>,
    ) -> FileRange {
        let sema = &self.sema;
        (|| {
            let precise_location = precise_location?;
            let root = sema.parse_or_expand(node.file_id);
            match root.covering_element(precise_location) {
                syntax::NodeOrToken::Node(it) => Some(sema.original_range(&it)),
                syntax::NodeOrToken::Token(it) => {
                    node.with_value(it).original_file_range_opt(sema.db)
                }
            }
        })()
        .unwrap_or_else(|| sema.diagnostics_display_range(*node))
        .into()
    }
}

/// Request parser level diagnostics for the given [`FileId`].
pub fn syntax_diagnostics(
    db: &RootDatabase,
    config: &DiagnosticsConfig,
    file_id: FileId,
) -> Vec<Diagnostic> {
    let _p = tracing::info_span!("syntax_diagnostics").entered();

    if config.disabled.contains("syntax-error") {
        return Vec::new();
    }

    let sema = Semantics::new(db);
    let file_id = sema
        .attach_first_edition(file_id)
        .unwrap_or_else(|| EditionedFileId::current_edition(file_id));

    // [#3434] Only take first 128 errors to prevent slowing down editor/ide, the number 128 is chosen arbitrarily.
    db.parse_errors(file_id)
        .as_deref()
        .into_iter()
        .flatten()
        .take(128)
        .map(|err| {
            Diagnostic::new(
                DiagnosticCode::SyntaxError,
                format!("Syntax Error: {err}"),
                FileRange { file_id: file_id.into(), range: err.range() },
            )
        })
        .collect()
}

/// Request semantic diagnostics for the given [`FileId`]. The produced diagnostics may point to other files
/// due to macros.
pub fn semantic_diagnostics(
    db: &RootDatabase,
    config: &DiagnosticsConfig,
    resolve: &AssistResolveStrategy,
    file_id: FileId,
) -> Vec<Diagnostic> {
    let _p = tracing::info_span!("semantic_diagnostics").entered();
    let sema = Semantics::new(db);
    let file_id = sema
        .attach_first_edition(file_id)
        .unwrap_or_else(|| EditionedFileId::current_edition(file_id));
    let mut res = Vec::new();

    let parse = sema.parse(file_id);

    // FIXME: This iterates the entire file which is a rather expensive operation.
    // We should implement these differently in some form?
    // Salsa caching + incremental re-parse would be better here
    for node in parse.syntax().descendants() {
        handlers::useless_braces::useless_braces(&mut res, file_id, &node);
        handlers::field_shorthand::field_shorthand(&mut res, file_id, &node);
        handlers::json_is_not_rust::json_in_items(
            &sema,
            &mut res,
            file_id,
            &node,
            config,
            file_id.edition(),
        );
    }

    let module = sema.file_to_module_def(file_id);

    let ctx = DiagnosticsContext { config, sema, resolve, edition: file_id.edition() };

    let mut diags = Vec::new();
    match module {
        // A bunch of parse errors in a file indicate some bigger structural parse changes in the
        // file, so we skip semantic diagnostics so we can show these faster.
        Some(m) => {
            if !db.parse_errors(file_id).as_deref().is_some_and(|es| es.len() >= 16) {
                m.diagnostics(db, &mut diags, config.style_lints);
            }
        }
        None => handlers::unlinked_file::unlinked_file(&ctx, &mut res, file_id.file_id()),
    }

    for diag in diags {
        let d = match diag {
            AnyDiagnostic::AwaitOutsideOfAsync(d) => handlers::await_outside_of_async::await_outside_of_async(&ctx, &d),
            AnyDiagnostic::ExpectedFunction(d) => handlers::expected_function::expected_function(&ctx, &d),
            AnyDiagnostic::InactiveCode(d) => match handlers::inactive_code::inactive_code(&ctx, &d) {
                Some(it) => it,
                None => continue,
            }
            AnyDiagnostic::IncoherentImpl(d) => handlers::incoherent_impl::incoherent_impl(&ctx, &d),
            AnyDiagnostic::IncorrectCase(d) => handlers::incorrect_case::incorrect_case(&ctx, &d),
            AnyDiagnostic::InvalidDeriveTarget(d) => handlers::invalid_derive_target::invalid_derive_target(&ctx, &d),
            AnyDiagnostic::MacroDefError(d) => handlers::macro_error::macro_def_error(&ctx, &d),
            AnyDiagnostic::MacroError(d) => handlers::macro_error::macro_error(&ctx, &d),
            AnyDiagnostic::MacroExpansionParseError(d) => {
                // FIXME: Point to the correct error span here, not just the macro-call name
                res.extend(d.errors.iter().take(16).map(|err| {
                    {
                        Diagnostic::new(
                            DiagnosticCode::SyntaxError,
                            format!("Syntax Error in Expansion: {err}"),
                            ctx.resolve_precise_location(&d.node.clone(), d.precise_location),
                        )
                    }
                    .experimental()
                }));
                continue;
            },
            AnyDiagnostic::MalformedDerive(d) => handlers::malformed_derive::malformed_derive(&ctx, &d),
            AnyDiagnostic::MismatchedArgCount(d) => handlers::mismatched_arg_count::mismatched_arg_count(&ctx, &d),
            AnyDiagnostic::MissingFields(d) => handlers::missing_fields::missing_fields(&ctx, &d),
            AnyDiagnostic::MissingMatchArms(d) => handlers::missing_match_arms::missing_match_arms(&ctx, &d),
            AnyDiagnostic::MissingUnsafe(d) => handlers::missing_unsafe::missing_unsafe(&ctx, &d),
            AnyDiagnostic::MovedOutOfRef(d) => handlers::moved_out_of_ref::moved_out_of_ref(&ctx, &d),
            AnyDiagnostic::NeedMut(d) => match handlers::mutability_errors::need_mut(&ctx, &d) {
                Some(it) => it,
                None => continue,
            },
            AnyDiagnostic::NonExhaustiveLet(d) => handlers::non_exhaustive_let::non_exhaustive_let(&ctx, &d),
            AnyDiagnostic::NoSuchField(d) => handlers::no_such_field::no_such_field(&ctx, &d),
            AnyDiagnostic::PrivateAssocItem(d) => handlers::private_assoc_item::private_assoc_item(&ctx, &d),
            AnyDiagnostic::PrivateField(d) => handlers::private_field::private_field(&ctx, &d),
            AnyDiagnostic::ReplaceFilterMapNextWithFindMap(d) => handlers::replace_filter_map_next_with_find_map::replace_filter_map_next_with_find_map(&ctx, &d),
            AnyDiagnostic::TraitImplIncorrectSafety(d) => handlers::trait_impl_incorrect_safety::trait_impl_incorrect_safety(&ctx, &d),
            AnyDiagnostic::TraitImplMissingAssocItems(d) => handlers::trait_impl_missing_assoc_item::trait_impl_missing_assoc_item(&ctx, &d),
            AnyDiagnostic::TraitImplRedundantAssocItems(d) => handlers::trait_impl_redundant_assoc_item::trait_impl_redundant_assoc_item(&ctx, &d),
            AnyDiagnostic::TraitImplOrphan(d) => handlers::trait_impl_orphan::trait_impl_orphan(&ctx, &d),
            AnyDiagnostic::TypedHole(d) => handlers::typed_hole::typed_hole(&ctx, &d),
            AnyDiagnostic::TypeMismatch(d) => handlers::type_mismatch::type_mismatch(&ctx, &d),
            AnyDiagnostic::UndeclaredLabel(d) => handlers::undeclared_label::undeclared_label(&ctx, &d),
            AnyDiagnostic::UnimplementedBuiltinMacro(d) => handlers::unimplemented_builtin_macro::unimplemented_builtin_macro(&ctx, &d),
            AnyDiagnostic::UnreachableLabel(d) => handlers::unreachable_label::unreachable_label(&ctx, &d),
            AnyDiagnostic::UnresolvedAssocItem(d) => handlers::unresolved_assoc_item::unresolved_assoc_item(&ctx, &d),
            AnyDiagnostic::UnresolvedExternCrate(d) => handlers::unresolved_extern_crate::unresolved_extern_crate(&ctx, &d),
            AnyDiagnostic::UnresolvedField(d) => handlers::unresolved_field::unresolved_field(&ctx, &d),
            AnyDiagnostic::UnresolvedIdent(d) => handlers::unresolved_ident::unresolved_ident(&ctx, &d),
            AnyDiagnostic::UnresolvedImport(d) => handlers::unresolved_import::unresolved_import(&ctx, &d),
            AnyDiagnostic::UnresolvedMacroCall(d) => handlers::unresolved_macro_call::unresolved_macro_call(&ctx, &d),
            AnyDiagnostic::UnresolvedMethodCall(d) => handlers::unresolved_method::unresolved_method(&ctx, &d),
            AnyDiagnostic::UnresolvedModule(d) => handlers::unresolved_module::unresolved_module(&ctx, &d),
            AnyDiagnostic::UnusedMut(d) => match handlers::mutability_errors::unused_mut(&ctx, &d) {
                Some(it) => it,
                None => continue,
            },
            AnyDiagnostic::UnusedVariable(d) => match handlers::unused_variables::unused_variables(&ctx, &d) {
                Some(it) => it,
                None => continue,
            },
            AnyDiagnostic::BreakOutsideOfLoop(d) => handlers::break_outside_of_loop::break_outside_of_loop(&ctx, &d),
            AnyDiagnostic::MismatchedTupleStructPatArgCount(d) => handlers::mismatched_arg_count::mismatched_tuple_struct_pat_arg_count(&ctx, &d),
            AnyDiagnostic::RemoveTrailingReturn(d) => match handlers::remove_trailing_return::remove_trailing_return(&ctx, &d) {
                Some(it) => it,
                None => continue,
            },
            AnyDiagnostic::RemoveUnnecessaryElse(d) => match handlers::remove_unnecessary_else::remove_unnecessary_else(&ctx, &d) {
                Some(it) => it,
                None => continue,
            },
        };
        res.push(d)
    }

    res.retain(|d| {
        !(ctx.config.disabled.contains(d.code.as_str())
            || ctx.config.disable_experimental && d.experimental)
    });

    let mut diagnostics_of_range = res
        .iter_mut()
        .filter_map(|it| {
            Some((
                it.main_node.map(|ptr| {
                    ptr.map(|node| node.to_node(&ctx.sema.parse_or_expand(ptr.file_id)))
                })?,
                it,
            ))
        })
        .collect::<FxHashMap<_, _>>();

    if diagnostics_of_range.is_empty() {
        return res;
    }

    let mut rustc_stack: FxHashMap<String, Vec<Severity>> = FxHashMap::default();
    let mut clippy_stack: FxHashMap<String, Vec<Severity>> = FxHashMap::default();

    // FIXME: This becomes quite expensive for big files
    handle_lint_attributes(
        &ctx.sema,
        parse.syntax(),
        &mut rustc_stack,
        &mut clippy_stack,
        &mut diagnostics_of_range,
        ctx.edition,
    );

    res.retain(|d| d.severity != Severity::Allow);

    res
}

/// Request both syntax and semantic diagnostics for the given [`FileId`].
pub fn full_diagnostics(
    db: &RootDatabase,
    config: &DiagnosticsConfig,
    resolve: &AssistResolveStrategy,
    file_id: FileId,
) -> Vec<Diagnostic> {
    let mut res = syntax_diagnostics(db, config, file_id);
    let sema = semantic_diagnostics(db, config, resolve, file_id);
    res.extend(sema);
    res
}

// `__RA_EVERY_LINT` is a fake lint group to allow every lint in proc macros

static RUSTC_LINT_GROUPS_DICT: LazyLock<FxHashMap<&str, Vec<&str>>> =
    LazyLock::new(|| build_group_dict(DEFAULT_LINT_GROUPS, &["warnings", "__RA_EVERY_LINT"], ""));

static CLIPPY_LINT_GROUPS_DICT: LazyLock<FxHashMap<&str, Vec<&str>>> =
    LazyLock::new(|| build_group_dict(CLIPPY_LINT_GROUPS, &["__RA_EVERY_LINT"], "clippy::"));

fn build_group_dict(
    lint_group: &'static [LintGroup],
    all_groups: &'static [&'static str],
    prefix: &'static str,
) -> FxHashMap<&'static str, Vec<&'static str>> {
    let mut r: FxHashMap<&str, Vec<&str>> = FxHashMap::default();
    for g in lint_group {
        for child in g.children {
            r.entry(child.strip_prefix(prefix).unwrap())
                .or_default()
                .push(g.lint.label.strip_prefix(prefix).unwrap());
        }
    }
    for (lint, groups) in r.iter_mut() {
        groups.push(lint);
        groups.extend_from_slice(all_groups);
    }
    r
}

fn handle_lint_attributes(
    sema: &Semantics<'_, RootDatabase>,
    root: &SyntaxNode,
    rustc_stack: &mut FxHashMap<String, Vec<Severity>>,
    clippy_stack: &mut FxHashMap<String, Vec<Severity>>,
    diagnostics_of_range: &mut FxHashMap<InFile<SyntaxNode>, &mut Diagnostic>,
    edition: Edition,
) {
    let _g = tracing::info_span!("handle_lint_attributes").entered();
    let file_id = sema.hir_file_for(root);
    let preorder = root.preorder();
    for ev in preorder {
        match ev {
            syntax::WalkEvent::Enter(node) => {
                for attr in node.children().filter_map(ast::Attr::cast) {
                    parse_lint_attribute(
                        attr,
                        rustc_stack,
                        clippy_stack,
                        |stack, severity| {
                            stack.push(severity);
                        },
                        edition,
                    );
                }
                if let Some(it) =
                    diagnostics_of_range.get_mut(&InFile { file_id, value: node.clone() })
                {
                    const EMPTY_LINTS: &[&str] = &[];
                    let (names, stack) = match it.code {
                        DiagnosticCode::RustcLint(name) => (
                            RUSTC_LINT_GROUPS_DICT.get(name).map_or(EMPTY_LINTS, |it| &**it),
                            &mut *rustc_stack,
                        ),
                        DiagnosticCode::Clippy(name) => (
                            CLIPPY_LINT_GROUPS_DICT.get(name).map_or(EMPTY_LINTS, |it| &**it),
                            &mut *clippy_stack,
                        ),
                        _ => continue,
                    };
                    for &name in names {
                        if let Some(s) = stack.get(name).and_then(|it| it.last()) {
                            it.severity = *s;
                        }
                    }
                }
                if let Some(item) = ast::Item::cast(node.clone()) {
                    if let Some(me) = sema.expand_attr_macro(&item) {
                        for stack in [&mut *rustc_stack, &mut *clippy_stack] {
                            stack
                                .entry("__RA_EVERY_LINT".to_owned())
                                .or_default()
                                .push(Severity::Allow);
                        }
                        handle_lint_attributes(
                            sema,
                            &me,
                            rustc_stack,
                            clippy_stack,
                            diagnostics_of_range,
                            edition,
                        );
                        for stack in [&mut *rustc_stack, &mut *clippy_stack] {
                            stack.entry("__RA_EVERY_LINT".to_owned()).or_default().pop();
                        }
                    }
                }
                if let Some(mc) = ast::MacroCall::cast(node) {
                    if let Some(me) = sema.expand(&mc) {
                        handle_lint_attributes(
                            sema,
                            &me,
                            rustc_stack,
                            clippy_stack,
                            diagnostics_of_range,
                            edition,
                        );
                    }
                }
            }
            syntax::WalkEvent::Leave(node) => {
                for attr in node.children().filter_map(ast::Attr::cast) {
                    parse_lint_attribute(
                        attr,
                        rustc_stack,
                        clippy_stack,
                        |stack, severity| {
                            if stack.pop() != Some(severity) {
                                never!("Mismatched serevity in walking lint attributes");
                            }
                        },
                        edition,
                    );
                }
            }
        }
    }
}

fn parse_lint_attribute(
    attr: ast::Attr,
    rustc_stack: &mut FxHashMap<String, Vec<Severity>>,
    clippy_stack: &mut FxHashMap<String, Vec<Severity>>,
    job: impl Fn(&mut Vec<Severity>, Severity),
    edition: Edition,
) {
    let Some((tag, args_tt)) = attr.as_simple_call() else {
        return;
    };
    let severity = match tag.as_str() {
        "allow" => Severity::Allow,
        "warn" => Severity::Warning,
        "forbid" | "deny" => Severity::Error,
        _ => return,
    };
    for lint in parse_tt_as_comma_sep_paths(args_tt, edition).into_iter().flatten() {
        if let Some(lint) = lint.as_single_name_ref() {
            job(rustc_stack.entry(lint.to_string()).or_default(), severity);
        }
        if let Some(tool) = lint.qualifier().and_then(|it| it.as_single_name_ref()) {
            if let Some(name_ref) = &lint.segment().and_then(|it| it.name_ref()) {
                if tool.to_string() == "clippy" {
                    job(clippy_stack.entry(name_ref.to_string()).or_default(), severity);
                }
            }
        }
    }
}

fn fix(id: &'static str, label: &str, source_change: SourceChange, target: TextRange) -> Assist {
    let mut res = unresolved_fix(id, label, target);
    res.source_change = Some(source_change);
    res
}

fn unresolved_fix(id: &'static str, label: &str, target: TextRange) -> Assist {
    assert!(!id.contains(' '));
    Assist {
        id: AssistId(id, AssistKind::QuickFix),
        label: Label::new(label.to_owned()),
        group: None,
        target,
        source_change: None,
        command: None,
    }
}

fn adjusted_display_range<N: AstNode>(
    ctx: &DiagnosticsContext<'_>,
    diag_ptr: InFile<AstPtr<N>>,
    adj: &dyn Fn(N) -> Option<TextRange>,
) -> FileRange {
    let source_file = ctx.sema.parse_or_expand(diag_ptr.file_id);
    let node = diag_ptr.value.to_node(&source_file);
    diag_ptr
        .with_value(adj(node).unwrap_or_else(|| diag_ptr.value.text_range()))
        .original_node_file_range_rooted(ctx.sema.db)
        .into()
}
