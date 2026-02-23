use std::iter;

use rustc_errors::pluralize;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::DefId;
use rustc_hir::{self as hir, LangItem, find_attr};
use rustc_infer::traits::util::elaborate;
use rustc_middle::ty::{self, Ty};
use rustc_session::{declare_lint, declare_lint_pass};
use rustc_span::{Span, Symbol, sym};
use tracing::instrument;

use crate::lints::{
    UnusedClosure, UnusedCoroutine, UnusedDef, UnusedDefSuggestion, UnusedOp, UnusedOpSuggestion,
    UnusedResult,
};
use crate::{LateContext, LateLintPass, LintContext};

declare_lint! {
    /// The `unused_must_use` lint detects unused result of a type flagged as
    /// `#[must_use]`.
    ///
    /// ### Example
    ///
    /// ```rust
    /// fn returns_result() -> Result<(), ()> {
    ///     Ok(())
    /// }
    ///
    /// fn main() {
    ///     returns_result();
    /// }
    /// ```
    ///
    /// {{produces}}
    ///
    /// ### Explanation
    ///
    /// The `#[must_use]` attribute is an indicator that it is a mistake to
    /// ignore the value. See [the reference] for more details.
    ///
    /// [the reference]: https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute
    pub UNUSED_MUST_USE,
    Warn,
    "unused result of a type flagged as `#[must_use]`",
    report_in_external_macro
}

declare_lint! {
    /// The `unused_results` lint checks for the unused result of an
    /// expression in a statement.
    ///
    /// ### Example
    ///
    /// ```rust,compile_fail
    /// #![deny(unused_results)]
    /// fn foo<T>() -> T { panic!() }
    ///
    /// fn main() {
    ///     foo::<usize>();
    /// }
    /// ```
    ///
    /// {{produces}}
    ///
    /// ### Explanation
    ///
    /// Ignoring the return value of a function may indicate a mistake. In
    /// cases were it is almost certain that the result should be used, it is
    /// recommended to annotate the function with the [`must_use` attribute].
    /// Failure to use such a return value will trigger the [`unused_must_use`
    /// lint] which is warn-by-default. The `unused_results` lint is
    /// essentially the same, but triggers for *all* return values.
    ///
    /// This lint is "allow" by default because it can be noisy, and may not be
    /// an actual problem. For example, calling the `remove` method of a `Vec`
    /// or `HashMap` returns the previous value, which you may not care about.
    /// Using this lint would require explicitly ignoring or discarding such
    /// values.
    ///
    /// [`must_use` attribute]: https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute
    /// [`unused_must_use` lint]: warn-by-default.html#unused-must-use
    pub UNUSED_RESULTS,
    Allow,
    "unused result of an expression in a statement"
}

declare_lint_pass!(UnusedResults => [UNUSED_MUST_USE, UNUSED_RESULTS]);

/// A path through a type to a `must_use` source. Contains useful info for the lint.
#[derive(Debug)]
pub enum MustUsePath {
    /// Suppress must_use checking.
    Suppressed,
    /// The root of the normal `must_use` lint with an optional message.
    Def(Span, DefId, Option<Symbol>),
    Boxed(Box<Self>),
    Pinned(Box<Self>),
    Opaque(Box<Self>),
    TraitObject(Box<Self>),
    TupleElement(Vec<(usize, Self)>),
    Array(Box<Self>, u64),
    /// The root of the unused_closures lint.
    Closure(Span),
    /// The root of the unused_coroutines lint.
    Coroutine(Span),
}

/// Returns `Some(path)` if `ty` should be considered as "`must_use`" in the context of `expr`
/// (`expr` is used to get the parent module, which can affect which types are considered uninhabited).
#[instrument(skip(cx, expr), level = "debug", ret)]
pub fn is_ty_must_use<'tcx>(
    cx: &LateContext<'tcx>,
    ty: Ty<'tcx>,
    expr: &hir::Expr<'_>,
    span: Span,
) -> Option<MustUsePath> {
    if ty.is_unit() {
        return Some(MustUsePath::Suppressed);
    }
    let parent_mod_did = cx.tcx.parent_module(expr.hir_id).to_def_id();
    let is_uninhabited =
        |t: Ty<'tcx>| !t.is_inhabited_from(cx.tcx, parent_mod_did, cx.typing_env());
    if is_uninhabited(ty) {
        return Some(MustUsePath::Suppressed);
    }

    match *ty.kind() {
        ty::Adt(..) if let Some(boxed) = ty.boxed_ty() => {
            is_ty_must_use(cx, boxed, expr, span).map(|inner| MustUsePath::Boxed(Box::new(inner)))
        }
        ty::Adt(def, args) if cx.tcx.is_lang_item(def.did(), LangItem::Pin) => {
            let pinned_ty = args.type_at(0);
            is_ty_must_use(cx, pinned_ty, expr, span)
                .map(|inner| MustUsePath::Pinned(Box::new(inner)))
        }
        // Suppress warnings on `Result<(), Uninhabited>` (e.g. `Result<(), !>`).
        ty::Adt(def, args)
            if cx.tcx.is_diagnostic_item(sym::Result, def.did())
                && args.type_at(0).is_unit()
                && is_uninhabited(args.type_at(1)) =>
        {
            Some(MustUsePath::Suppressed)
        }
        // Suppress warnings on `ControlFlow<Uninhabited, ()>` (e.g. `ControlFlow<!, ()>`).
        ty::Adt(def, args)
            if cx.tcx.is_diagnostic_item(sym::ControlFlow, def.did())
                && args.type_at(1).is_unit()
                && is_uninhabited(args.type_at(0)) =>
        {
            Some(MustUsePath::Suppressed)
        }
        ty::Adt(def, _) => is_def_must_use(cx, def.did(), span),
        ty::Alias(ty::Opaque | ty::Projection, ty::AliasTy { def_id: def, .. }) => {
            elaborate(cx.tcx, cx.tcx.explicit_item_self_bounds(def).iter_identity_copied())
                // We only care about self bounds for the impl-trait
                .filter_only_self()
                .find_map(|(pred, _span)| {
                    // We only look at the `DefId`, so it is safe to skip the binder here.
                    if let ty::ClauseKind::Trait(ref poly_trait_predicate) =
                        pred.kind().skip_binder()
                    {
                        let def_id = poly_trait_predicate.trait_ref.def_id;

                        is_def_must_use(cx, def_id, span)
                    } else {
                        None
                    }
                })
                .map(|inner| MustUsePath::Opaque(Box::new(inner)))
        }
        ty::Dynamic(binders, _) => binders.iter().find_map(|predicate| {
            if let ty::ExistentialPredicate::Trait(ref trait_ref) = predicate.skip_binder() {
                let def_id = trait_ref.def_id;
                is_def_must_use(cx, def_id, span)
                    .map(|inner| MustUsePath::TraitObject(Box::new(inner)))
            } else {
                None
            }
        }),
        ty::Tuple(tys) => {
            let elem_exprs = if let hir::ExprKind::Tup(elem_exprs) = expr.kind {
                debug_assert_eq!(elem_exprs.len(), tys.len());
                elem_exprs
            } else {
                &[]
            };

            // Default to `expr`.
            let elem_exprs = elem_exprs.iter().chain(iter::repeat(expr));

            let nested_must_use = tys
                .iter()
                .zip(elem_exprs)
                .enumerate()
                .filter_map(|(i, (ty, expr))| {
                    is_ty_must_use(cx, ty, expr, expr.span).map(|path| (i, path))
                })
                .collect::<Vec<_>>();

            if !nested_must_use.is_empty() {
                Some(MustUsePath::TupleElement(nested_must_use))
            } else {
                None
            }
        }
        ty::Array(ty, len) => match len.try_to_target_usize(cx.tcx) {
            // If the array is empty we don't lint, to avoid false positives
            Some(0) | None => None,
            // If the array is definitely non-empty, we can do `#[must_use]` checking.
            Some(len) => is_ty_must_use(cx, ty, expr, span)
                .map(|inner| MustUsePath::Array(Box::new(inner), len)),
        },
        ty::Closure(..) | ty::CoroutineClosure(..) => Some(MustUsePath::Closure(span)),
        ty::Coroutine(def_id, ..) => {
            // async fn should be treated as "implementor of `Future`"
            let must_use = if cx.tcx.coroutine_is_async(def_id) {
                let def_id = cx.tcx.lang_items().future_trait()?;
                is_def_must_use(cx, def_id, span).map(|inner| MustUsePath::Opaque(Box::new(inner)))
            } else {
                None
            };
            must_use.or(Some(MustUsePath::Coroutine(span)))
        }
        _ => None,
    }
}

impl<'tcx> LateLintPass<'tcx> for UnusedResults {
    fn check_stmt(&mut self, cx: &LateContext<'_>, s: &hir::Stmt<'_>) {
        let hir::StmtKind::Semi(mut expr) = s.kind else {
            return;
        };

        let mut expr_is_from_block = false;
        while let hir::ExprKind::Block(blk, ..) = expr.kind
            && let hir::Block { expr: Some(e), .. } = blk
        {
            expr = e;
            expr_is_from_block = true;
        }

        if let hir::ExprKind::Ret(..) = expr.kind {
            return;
        }

        if let hir::ExprKind::Match(await_expr, _arms, hir::MatchSource::AwaitDesugar) = expr.kind
            && let ty = cx.typeck_results().expr_ty(await_expr)
            && let ty::Alias(ty::Opaque, ty::AliasTy { def_id: future_def_id, .. }) = ty.kind()
            && cx.tcx.ty_is_opaque_future(ty)
            && let async_fn_def_id = cx.tcx.parent(*future_def_id)
            && matches!(cx.tcx.def_kind(async_fn_def_id), DefKind::Fn | DefKind::AssocFn)
            // Check that this `impl Future` actually comes from an `async fn`
            && cx.tcx.asyncness(async_fn_def_id).is_async()
            && check_must_use_def(
                cx,
                async_fn_def_id,
                expr.span,
                "output of future returned by ",
                "",
                expr_is_from_block,
            )
        {
            // We have a bare `foo().await;` on an opaque type from an async function that was
            // annotated with `#[must_use]`.
            return;
        }

        let ty = cx.typeck_results().expr_ty(expr);

        let must_use_result = is_ty_must_use(cx, ty, expr, expr.span);
        let type_lint_emitted_or_suppressed = match must_use_result {
            Some(path) => {
                emit_must_use_untranslated(cx, &path, "", "", 1, false, expr_is_from_block);
                true
            }
            None => false,
        };

        let fn_warned = check_fn_must_use(cx, expr, expr_is_from_block);

        if !fn_warned && type_lint_emitted_or_suppressed {
            // We don't warn about unused unit or uninhabited types.
            // (See https://github.com/rust-lang/rust/issues/43806 for details.)
            return;
        }

        let must_use_op = match expr.kind {
            // Hardcoding operators here seemed more expedient than the
            // refactoring that would be needed to look up the `#[must_use]`
            // attribute which does exist on the comparison trait methods
            hir::ExprKind::Binary(bin_op, ..) => match bin_op.node {
                hir::BinOpKind::Eq
                | hir::BinOpKind::Lt
                | hir::BinOpKind::Le
                | hir::BinOpKind::Ne
                | hir::BinOpKind::Ge
                | hir::BinOpKind::Gt => Some("comparison"),
                hir::BinOpKind::Add
                | hir::BinOpKind::Sub
                | hir::BinOpKind::Div
                | hir::BinOpKind::Mul
                | hir::BinOpKind::Rem => Some("arithmetic operation"),
                hir::BinOpKind::And | hir::BinOpKind::Or => Some("logical operation"),
                hir::BinOpKind::BitXor
                | hir::BinOpKind::BitAnd
                | hir::BinOpKind::BitOr
                | hir::BinOpKind::Shl
                | hir::BinOpKind::Shr => Some("bitwise operation"),
            },
            hir::ExprKind::AddrOf(..) => Some("borrow"),
            hir::ExprKind::OffsetOf(..) => Some("`offset_of` call"),
            hir::ExprKind::Unary(..) => Some("unary operation"),
            // The `offset_of` macro wraps its contents inside a `const` block.
            hir::ExprKind::ConstBlock(block) => {
                let body = cx.tcx.hir_body(block.body);
                if let hir::ExprKind::Block(block, _) = body.value.kind
                    && let Some(expr) = block.expr
                    && let hir::ExprKind::OffsetOf(..) = expr.kind
                {
                    Some("`offset_of` call")
                } else {
                    None
                }
            }
            _ => None,
        };

        let mut op_warned = false;

        if let Some(must_use_op) = must_use_op {
            let span = expr.span.find_ancestor_not_from_macro().unwrap_or(expr.span);
            cx.emit_span_lint(
                UNUSED_MUST_USE,
                expr.span,
                UnusedOp {
                    op: must_use_op,
                    label: expr.span,
                    suggestion: if expr_is_from_block {
                        UnusedOpSuggestion::BlockTailExpr {
                            before_span: span.shrink_to_lo(),
                            after_span: span.shrink_to_hi(),
                        }
                    } else {
                        UnusedOpSuggestion::NormalExpr { span: span.shrink_to_lo() }
                    },
                },
            );
            op_warned = true;
        }

        if !(type_lint_emitted_or_suppressed || fn_warned || op_warned) {
            cx.emit_span_lint(UNUSED_RESULTS, s.span, UnusedResult { ty });
        }
    }
}

/// Checks if `expr` is a \[method\] call expression marked as `#[must_use]` and emits a lint if so.
/// Returns `true` if the lint has been emitted.
fn check_fn_must_use(cx: &LateContext<'_>, expr: &hir::Expr<'_>, expr_is_from_block: bool) -> bool {
    let maybe_def_id = match expr.kind {
        hir::ExprKind::Call(callee, _) => {
            match callee.kind {
                hir::ExprKind::Path(ref qpath) => {
                    match cx.qpath_res(qpath, callee.hir_id) {
                        Res::Def(DefKind::Fn | DefKind::AssocFn, def_id) => Some(def_id),
                        // `Res::Local` if it was a closure, for which we
                        // do not currently support must-use linting
                        _ => None,
                    }
                }
                _ => None,
            }
        }
        hir::ExprKind::MethodCall(..) => cx.typeck_results().type_dependent_def_id(expr.hir_id),
        _ => None,
    };
    if let Some(def_id) = maybe_def_id {
        check_must_use_def(cx, def_id, expr.span, "return value of ", "", expr_is_from_block)
    } else {
        false
    }
}

fn is_def_must_use(cx: &LateContext<'_>, def_id: DefId, span: Span) -> Option<MustUsePath> {
    if let Some(reason) = find_attr!(
        cx.tcx, def_id,
        MustUse { reason, .. } => reason
    ) {
        // check for #[must_use = "..."]
        Some(MustUsePath::Def(span, def_id, *reason))
    } else {
        None
    }
}

/// Returns whether further errors should be suppressed because a lint has been emitted.
fn check_must_use_def(
    cx: &LateContext<'_>,
    def_id: DefId,
    span: Span,
    descr_pre_path: &str,
    descr_post_path: &str,
    expr_is_from_block: bool,
) -> bool {
    is_def_must_use(cx, def_id, span)
        .map(|must_use_path| {
            emit_must_use_untranslated(
                cx,
                &must_use_path,
                descr_pre_path,
                descr_post_path,
                1,
                false,
                expr_is_from_block,
            )
        })
        .is_some()
}

#[instrument(skip(cx), level = "debug")]
fn emit_must_use_untranslated(
    cx: &LateContext<'_>,
    path: &MustUsePath,
    descr_pre: &str,
    descr_post: &str,
    plural_len: usize,
    is_inner: bool,
    expr_is_from_block: bool,
) {
    let plural_suffix = pluralize!(plural_len);

    match path {
        MustUsePath::Suppressed => {}
        MustUsePath::Boxed(path) => {
            let descr_pre = &format!("{descr_pre}boxed ");
            emit_must_use_untranslated(
                cx,
                path,
                descr_pre,
                descr_post,
                plural_len,
                true,
                expr_is_from_block,
            );
        }
        MustUsePath::Pinned(path) => {
            let descr_pre = &format!("{descr_pre}pinned ");
            emit_must_use_untranslated(
                cx,
                path,
                descr_pre,
                descr_post,
                plural_len,
                true,
                expr_is_from_block,
            );
        }
        MustUsePath::Opaque(path) => {
            let descr_pre = &format!("{descr_pre}implementer{plural_suffix} of ");
            emit_must_use_untranslated(
                cx,
                path,
                descr_pre,
                descr_post,
                plural_len,
                true,
                expr_is_from_block,
            );
        }
        MustUsePath::TraitObject(path) => {
            let descr_post = &format!(" trait object{plural_suffix}{descr_post}");
            emit_must_use_untranslated(
                cx,
                path,
                descr_pre,
                descr_post,
                plural_len,
                true,
                expr_is_from_block,
            );
        }
        MustUsePath::TupleElement(elems) => {
            for (index, path) in elems {
                let descr_post = &format!(" in tuple element {index}");
                emit_must_use_untranslated(
                    cx,
                    path,
                    descr_pre,
                    descr_post,
                    plural_len,
                    true,
                    expr_is_from_block,
                );
            }
        }
        MustUsePath::Array(path, len) => {
            let descr_pre = &format!("{descr_pre}array{plural_suffix} of ");
            emit_must_use_untranslated(
                cx,
                path,
                descr_pre,
                descr_post,
                plural_len.saturating_add(usize::try_from(*len).unwrap_or(usize::MAX)),
                true,
                expr_is_from_block,
            );
        }
        MustUsePath::Closure(span) => {
            cx.emit_span_lint(
                UNUSED_MUST_USE,
                *span,
                UnusedClosure { count: plural_len, pre: descr_pre, post: descr_post },
            );
        }
        MustUsePath::Coroutine(span) => {
            cx.emit_span_lint(
                UNUSED_MUST_USE,
                *span,
                UnusedCoroutine { count: plural_len, pre: descr_pre, post: descr_post },
            );
        }
        MustUsePath::Def(span, def_id, reason) => {
            let ancenstor_span = span.find_ancestor_not_from_macro().unwrap_or(*span);
            let is_redundant_let_ignore = cx
                .sess()
                .source_map()
                .span_to_prev_source(ancenstor_span)
                .ok()
                .map(|prev| prev.trim_end().ends_with("let _ ="))
                .unwrap_or(false);
            let suggestion_span = if is_redundant_let_ignore { *span } else { ancenstor_span };
            cx.emit_span_lint(
                UNUSED_MUST_USE,
                ancenstor_span,
                UnusedDef {
                    pre: descr_pre,
                    post: descr_post,
                    cx,
                    def_id: *def_id,
                    note: *reason,
                    suggestion: (!is_inner).then_some(if expr_is_from_block {
                        UnusedDefSuggestion::BlockTailExpr {
                            before_span: suggestion_span.shrink_to_lo(),
                            after_span: suggestion_span.shrink_to_hi(),
                        }
                    } else {
                        UnusedDefSuggestion::NormalExpr { span: suggestion_span.shrink_to_lo() }
                    }),
                },
            );
        }
    }
}
