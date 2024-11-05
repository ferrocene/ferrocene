//! Implementation of "binding mode" inlay hints:
//! ```no_run
//! let /* & */ (/* ref */ x,) = &(0,);
//! ```
use hir::Mutability;
use ide_db::famous_defs::FamousDefs;

use span::EditionedFileId;
use syntax::ast::{self, AstNode};

use crate::{InlayHint, InlayHintPosition, InlayHintsConfig, InlayKind};

pub(super) fn hints(
    acc: &mut Vec<InlayHint>,
    FamousDefs(sema, _): &FamousDefs<'_, '_>,
    config: &InlayHintsConfig,
    _file_id: EditionedFileId,
    pat: &ast::Pat,
) -> Option<()> {
    if !config.binding_mode_hints {
        return None;
    }

    let outer_paren_pat = pat
        .syntax()
        .ancestors()
        .skip(1)
        .map_while(ast::Pat::cast)
        .map_while(|pat| match pat {
            ast::Pat::ParenPat(pat) => Some(pat),
            _ => None,
        })
        .last();
    let range = outer_paren_pat.as_ref().map_or_else(
        || match pat {
            // for ident patterns that @ bind a name, render the un-ref patterns in front of the inner pattern
            // instead of the name as that makes it more clear and doesn't really change the outcome
            ast::Pat::IdentPat(it) => {
                it.pat().map_or_else(|| it.syntax().text_range(), |it| it.syntax().text_range())
            }
            it => it.syntax().text_range(),
        },
        |it| it.syntax().text_range(),
    );
    let pattern_adjustments = sema.pattern_adjustments(pat);
    pattern_adjustments.iter().for_each(|ty| {
        let reference = ty.is_reference();
        let mut_reference = ty.is_mutable_reference();
        let r = match (reference, mut_reference) {
            (true, true) => "&mut",
            (true, false) => "&",
            _ => return,
        };
        acc.push(InlayHint {
            range,
            kind: InlayKind::BindingMode,
            label: r.into(),
            text_edit: None,
            position: InlayHintPosition::Before,
            pad_left: false,
            pad_right: mut_reference,
            resolve_parent: Some(pat.syntax().text_range()),
        });
    });
    match pat {
        ast::Pat::IdentPat(pat) if pat.ref_token().is_none() && pat.mut_token().is_none() => {
            let bm = sema.binding_mode_of_pat(pat)?;
            let bm = match bm {
                hir::BindingMode::Move => return None,
                hir::BindingMode::Ref(Mutability::Mut) => "ref mut",
                hir::BindingMode::Ref(Mutability::Shared) => "ref",
            };
            acc.push(InlayHint {
                range: pat.syntax().text_range(),
                kind: InlayKind::BindingMode,
                label: bm.into(),
                text_edit: None,
                position: InlayHintPosition::Before,
                pad_left: false,
                pad_right: true,
                resolve_parent: Some(pat.syntax().text_range()),
            });
        }
        ast::Pat::OrPat(pat) if !pattern_adjustments.is_empty() && outer_paren_pat.is_none() => {
            acc.push(InlayHint::opening_paren_before(
                InlayKind::BindingMode,
                pat.syntax().text_range(),
            ));
            acc.push(InlayHint::closing_paren_after(
                InlayKind::BindingMode,
                pat.syntax().text_range(),
            ));
        }
        _ => (),
    }

    Some(())
}

#[cfg(test)]
mod tests {
    use crate::{
        inlay_hints::tests::{check_with_config, DISABLED_CONFIG},
        InlayHintsConfig,
    };

    #[test]
    fn hints_binding_modes() {
        check_with_config(
            InlayHintsConfig { binding_mode_hints: true, ..DISABLED_CONFIG },
            r#"
fn __(
    (x,): (u32,),
    (x,): &(u32,),
  //^^^^&
   //^ ref
    (x,): &mut (u32,)
  //^^^^&mut
   //^ ref mut
) {
    let (x,) = (0,);
    let (x,) = &(0,);
      //^^^^ &
       //^ ref
    let (x,) = &mut (0,);
      //^^^^ &mut
       //^ ref mut
    let &mut (x,) = &mut (0,);
    let (ref mut x,) = &mut (0,);
      //^^^^^^^^^^^^ &mut
    let &mut (ref mut x,) = &mut (0,);
    let (mut x,) = &mut (0,);
      //^^^^^^^^ &mut
    match (0,) {
        (x,) => ()
    }
    match &(0,) {
        (x,) | (x,) => (),
      //^^^^^^^^^^^&
       //^ ref
              //^ ref
      //^^^^^^^^^^^(
      //^^^^^^^^^^^)
        ((x,) | (x,)) => (),
      //^^^^^^^^^^^^^&
        //^ ref
               //^ ref
    }
    match &mut (0,) {
        (x,) => ()
      //^^^^ &mut
       //^ ref mut
    }
}"#,
        );
    }

    #[test]
    fn hints_binding_modes_complex_ident_pat() {
        check_with_config(
            InlayHintsConfig { binding_mode_hints: true, ..DISABLED_CONFIG },
            r#"
struct Struct {
    field: &'static str,
}
fn foo(s @ Struct { field, .. }: &Struct) {}
         //^^^^^^^^^^^^^^^^^^^^&
                  //^^^^^ref
"#,
        );
    }
}
