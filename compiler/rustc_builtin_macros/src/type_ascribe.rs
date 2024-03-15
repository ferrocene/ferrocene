use rustc_ast::ptr::P;
use rustc_ast::tokenstream::TokenStream;
use rustc_ast::{token, Expr, ExprKind, Ty};
use rustc_errors::PResult;
use rustc_expand::base::{DummyResult, ExpandResult, ExtCtxt, MacEager, MacroExpanderResult};
use rustc_span::Span;

pub fn expand_type_ascribe(
    cx: &mut ExtCtxt<'_>,
    span: Span,
    tts: TokenStream,
) -> MacroExpanderResult<'static> {
    let (expr, ty) = match parse_ascribe(cx, tts) {
        Ok(parsed) => parsed,
        Err(err) => {
            let guar = err.emit();
            return ExpandResult::Ready(DummyResult::any(span, guar));
        }
    };

    let asc_expr = cx.expr(span, ExprKind::Type(expr, ty));

    ExpandResult::Ready(MacEager::expr(asc_expr))
}

fn parse_ascribe<'a>(cx: &mut ExtCtxt<'a>, stream: TokenStream) -> PResult<'a, (P<Expr>, P<Ty>)> {
    let mut parser = cx.new_parser_from_tts(stream);

    let expr = parser.parse_expr()?;
    parser.expect(&token::Comma)?;

    let ty = parser.parse_ty()?;

    Ok((expr, ty))
}
