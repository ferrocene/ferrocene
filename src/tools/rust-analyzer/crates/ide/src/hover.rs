mod render;

#[cfg(test)]
mod tests;

use std::{iter, ops::Not};

use either::Either;
use hir::{db::DefDatabase, DescendPreference, HasCrate, HasSource, LangItem, Semantics};
use ide_db::{
    base_db::FileRange,
    defs::{Definition, IdentClass, NameRefClass, OperatorClass},
    famous_defs::FamousDefs,
    helpers::pick_best_token,
    FxIndexSet, RootDatabase,
};
use itertools::Itertools;
use syntax::{ast, match_ast, AstNode, AstToken, SyntaxKind::*, SyntaxNode, T};

use crate::{
    doc_links::token_as_doc_comment,
    markdown_remove::remove_markdown,
    markup::Markup,
    navigation_target::UpmappingResult,
    runnables::{runnable_fn, runnable_mod},
    FileId, FilePosition, NavigationTarget, RangeInfo, Runnable, TryToNav,
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HoverConfig {
    pub links_in_hover: bool,
    pub memory_layout: Option<MemoryLayoutHoverConfig>,
    pub documentation: bool,
    pub keywords: bool,
    pub format: HoverDocFormat,
    pub max_trait_assoc_items_count: Option<usize>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct MemoryLayoutHoverConfig {
    pub size: Option<MemoryLayoutHoverRenderKind>,
    pub offset: Option<MemoryLayoutHoverRenderKind>,
    pub alignment: Option<MemoryLayoutHoverRenderKind>,
    pub niches: bool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MemoryLayoutHoverRenderKind {
    Decimal,
    Hexadecimal,
    Both,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HoverDocFormat {
    Markdown,
    PlainText,
}

#[derive(Debug, Clone)]
pub enum HoverAction {
    Runnable(Runnable),
    Implementation(FilePosition),
    Reference(FilePosition),
    GoToType(Vec<HoverGotoTypeData>),
}

impl HoverAction {
    fn goto_type_from_targets(db: &RootDatabase, targets: Vec<hir::ModuleDef>) -> Option<Self> {
        let targets = targets
            .into_iter()
            .filter_map(|it| {
                Some(HoverGotoTypeData {
                    mod_path: render::path(
                        db,
                        it.module(db)?,
                        it.name(db).map(|name| name.display(db).to_string()),
                    ),
                    nav: it.try_to_nav(db)?.call_site(),
                })
            })
            .collect::<Vec<_>>();
        targets.is_empty().not().then_some(HoverAction::GoToType(targets))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct HoverGotoTypeData {
    pub mod_path: String,
    pub nav: NavigationTarget,
}

/// Contains the results when hovering over an item
#[derive(Debug, Default)]
pub struct HoverResult {
    pub markup: Markup,
    pub actions: Vec<HoverAction>,
}

// Feature: Hover
//
// Shows additional information, like the type of an expression or the documentation for a definition when "focusing" code.
// Focusing is usually hovering with a mouse, but can also be triggered with a shortcut.
//
// image::https://user-images.githubusercontent.com/48062697/113020658-b5f98b80-917a-11eb-9f88-3dbc27320c95.gif[]
pub(crate) fn hover(
    db: &RootDatabase,
    frange @ FileRange { file_id, range }: FileRange,
    config: &HoverConfig,
) -> Option<RangeInfo<HoverResult>> {
    let sema = &hir::Semantics::new(db);
    let file = sema.parse(file_id).syntax().clone();
    let mut res = if range.is_empty() {
        hover_simple(sema, FilePosition { file_id, offset: range.start() }, file, config)
    } else {
        hover_ranged(sema, frange, file, config)
    }?;

    if let HoverDocFormat::PlainText = config.format {
        res.info.markup = remove_markdown(res.info.markup.as_str()).into();
    }
    Some(res)
}

#[allow(clippy::field_reassign_with_default)]
fn hover_simple(
    sema: &Semantics<'_, RootDatabase>,
    FilePosition { file_id, offset }: FilePosition,
    file: SyntaxNode,
    config: &HoverConfig,
) -> Option<RangeInfo<HoverResult>> {
    let original_token = pick_best_token(file.token_at_offset(offset), |kind| match kind {
        IDENT
        | INT_NUMBER
        | LIFETIME_IDENT
        | T![self]
        | T![super]
        | T![crate]
        | T![Self]
        | T![_] => 4,
        // index and prefix ops and closure pipe
        T!['['] | T![']'] | T![?] | T![*] | T![-] | T![!] | T![|] => 3,
        kind if kind.is_keyword() => 2,
        T!['('] | T![')'] => 2,
        kind if kind.is_trivia() => 0,
        _ => 1,
    })?;

    if let Some(doc_comment) = token_as_doc_comment(&original_token) {
        cov_mark::hit!(no_highlight_on_comment_hover);
        return doc_comment.get_definition_with_descend_at(sema, offset, |def, node, range| {
            let res = hover_for_definition(sema, file_id, def, &node, config);
            Some(RangeInfo::new(range, res))
        });
    }

    if let Some((range, resolution)) =
        sema.check_for_format_args_template(original_token.clone(), offset)
    {
        let res = hover_for_definition(
            sema,
            file_id,
            Definition::from(resolution?),
            &original_token.parent()?,
            config,
        );
        return Some(RangeInfo::new(range, res));
    }

    let in_attr = original_token
        .parent_ancestors()
        .filter_map(ast::Item::cast)
        .any(|item| sema.is_attr_macro_call(&item))
        && !matches!(
            original_token.parent().and_then(ast::TokenTree::cast),
            Some(tt) if tt.syntax().ancestors().any(|it| ast::Meta::can_cast(it.kind()))
        );

    // prefer descending the same token kind in attribute expansions, in normal macros text
    // equivalency is more important
    let descended = sema.descend_into_macros(
        if in_attr { DescendPreference::SameKind } else { DescendPreference::SameText },
        original_token.clone(),
    );
    let descended = || descended.iter();

    let result = descended()
        // try lint hover
        .find_map(|token| {
            // FIXME: Definition should include known lints and the like instead of having this special case here
            let attr = token.parent_ancestors().find_map(ast::Attr::cast)?;
            render::try_for_lint(&attr, token)
        })
        // try definitions
        .or_else(|| {
            descended()
                .filter_map(|token| {
                    let node = token.parent()?;
                    match IdentClass::classify_node(sema, &node)? {
                        // It's better for us to fall back to the keyword hover here,
                        // rendering poll is very confusing
                        IdentClass::Operator(OperatorClass::Await(_)) => None,

                        IdentClass::NameRefClass(NameRefClass::ExternCrateShorthand {
                            decl,
                            ..
                        }) => Some(vec![(Definition::ExternCrateDecl(decl), node)]),

                        class => Some(
                            class
                                .definitions()
                                .into_iter()
                                .zip(iter::repeat(node))
                                .collect::<Vec<_>>(),
                        ),
                    }
                })
                .flatten()
                .unique_by(|&(def, _)| def)
                .map(|(def, node)| hover_for_definition(sema, file_id, def, &node, config))
                .reduce(|mut acc: HoverResult, HoverResult { markup, actions }| {
                    acc.actions.extend(actions);
                    acc.markup = Markup::from(format!("{}\n---\n{markup}", acc.markup));
                    acc
                })
        })
        // try keywords
        .or_else(|| descended().find_map(|token| render::keyword(sema, config, token)))
        // try _ hovers
        .or_else(|| descended().find_map(|token| render::underscore(sema, config, token)))
        // try rest pattern hover
        .or_else(|| {
            descended().find_map(|token| {
                if token.kind() != DOT2 {
                    return None;
                }

                let rest_pat = token.parent().and_then(ast::RestPat::cast)?;
                let record_pat_field_list =
                    rest_pat.syntax().parent().and_then(ast::RecordPatFieldList::cast)?;

                let record_pat =
                    record_pat_field_list.syntax().parent().and_then(ast::RecordPat::cast)?;

                Some(render::struct_rest_pat(sema, config, &record_pat))
            })
        })
        // try () call hovers
        .or_else(|| {
            descended().find_map(|token| {
                if token.kind() != T!['('] && token.kind() != T![')'] {
                    return None;
                }
                let arg_list = token.parent().and_then(ast::ArgList::cast)?.syntax().parent()?;
                let call_expr = syntax::match_ast! {
                    match arg_list {
                        ast::CallExpr(expr) => expr.into(),
                        ast::MethodCallExpr(expr) => expr.into(),
                        _ => return None,
                    }
                };
                render::type_info_of(sema, config, &Either::Left(call_expr))
            })
        })
        // try closure
        .or_else(|| {
            descended().find_map(|token| {
                if token.kind() != T![|] {
                    return None;
                }
                let c = token.parent().and_then(|x| x.parent()).and_then(ast::ClosureExpr::cast)?;
                render::closure_expr(sema, config, c)
            })
        })
        // tokens
        .or_else(|| {
            let mut res = HoverResult::default();
            match_ast! {
                match original_token {
                    ast::String(string) => {
                        res.markup = Markup::fenced_block_text(format_args!("{}", string.value()?));
                    },
                    ast::ByteString(string) => {
                        res.markup = Markup::fenced_block_text(format_args!("{:?}", string.value()?));
                    },
                    ast::CString(string) => {
                        let val = string.value()?;
                        res.markup = Markup::fenced_block_text(format_args!("{}", std::str::from_utf8(val.as_ref()).ok()?));
                    },
                    ast::Char(char) => {
                        let mut res = HoverResult::default();
                        res.markup = Markup::fenced_block_text(format_args!("{}", char.value()?));
                    },
                    ast::Byte(byte) => {
                        res.markup = Markup::fenced_block_text(format_args!("0x{:X}", byte.value()?));
                    },
                    ast::FloatNumber(num) => {
                        res.markup = if num.suffix() == Some("f32") {
                            match num.value_f32() {
                                Ok(num) => {
                                    Markup::fenced_block_text(format_args!("{num} (bits: 0x{:X})", num.to_bits()))
                                },
                                Err(e) => {
                                    Markup::fenced_block_text(format_args!("{e}"))
                                },
                            }
                        } else {
                            match num.value() {
                                Ok(num) => {
                                    Markup::fenced_block_text(format_args!("{num} (bits: 0x{:X})", num.to_bits()))
                                },
                                Err(e) => {
                                    Markup::fenced_block_text(format_args!("{e}"))
                                },
                            }
                        };
                    },
                    ast::IntNumber(num) => {
                        res.markup = match num.value() {
                            Ok(num) => {
                                Markup::fenced_block_text(format_args!("{num} (0x{num:X}|0b{num:b})"))
                            },
                            Err(e) => {
                                Markup::fenced_block_text(format_args!("{e}"))
                            },
                        };
                    },
                    _ => return None
                }
            }
            Some(res)
        });

    result.map(|mut res: HoverResult| {
        res.actions = dedupe_or_merge_hover_actions(res.actions);
        RangeInfo::new(original_token.text_range(), res)
    })
}

fn hover_ranged(
    sema: &Semantics<'_, RootDatabase>,
    FileRange { range, .. }: FileRange,
    file: SyntaxNode,
    config: &HoverConfig,
) -> Option<RangeInfo<HoverResult>> {
    // FIXME: make this work in attributes
    let expr_or_pat = file
        .covering_element(range)
        .ancestors()
        .take_while(|it| ast::MacroCall::can_cast(it.kind()) || !ast::Item::can_cast(it.kind()))
        .find_map(Either::<ast::Expr, ast::Pat>::cast)?;
    let res = match &expr_or_pat {
        Either::Left(ast::Expr::TryExpr(try_expr)) => render::try_expr(sema, config, try_expr),
        Either::Left(ast::Expr::PrefixExpr(prefix_expr))
            if prefix_expr.op_kind() == Some(ast::UnaryOp::Deref) =>
        {
            render::deref_expr(sema, config, prefix_expr)
        }
        _ => None,
    };
    let res = res.or_else(|| render::type_info_of(sema, config, &expr_or_pat));
    res.map(|it| {
        let range = match expr_or_pat {
            Either::Left(it) => it.syntax().text_range(),
            Either::Right(it) => it.syntax().text_range(),
        };
        RangeInfo::new(range, it)
    })
}

// FIXME: Why is this pub(crate)?
pub(crate) fn hover_for_definition(
    sema: &Semantics<'_, RootDatabase>,
    file_id: FileId,
    def: Definition,
    scope_node: &SyntaxNode,
    config: &HoverConfig,
) -> HoverResult {
    let famous_defs = match &def {
        Definition::BuiltinType(_) => sema.scope(scope_node).map(|it| FamousDefs(sema, it.krate())),
        _ => None,
    };

    let db = sema.db;
    let def_ty = match def {
        Definition::Local(it) => Some(it.ty(db)),
        Definition::GenericParam(hir::GenericParam::ConstParam(it)) => Some(it.ty(db)),
        Definition::GenericParam(hir::GenericParam::TypeParam(it)) => Some(it.ty(db)),
        Definition::Field(field) => Some(field.ty(db)),
        Definition::TupleField(it) => Some(it.ty(db)),
        Definition::Function(it) => Some(it.ty(db)),
        Definition::Adt(it) => Some(it.ty(db)),
        Definition::Const(it) => Some(it.ty(db)),
        Definition::Static(it) => Some(it.ty(db)),
        Definition::TypeAlias(it) => Some(it.ty(db)),
        Definition::BuiltinType(it) => Some(it.ty(db)),
        _ => None,
    };
    let notable_traits = def_ty.map(|ty| notable_traits(db, &ty)).unwrap_or_default();

    let markup = render::definition(sema.db, def, famous_defs.as_ref(), &notable_traits, config);
    HoverResult {
        markup: render::process_markup(sema.db, def, &markup, config),
        actions: [
            show_implementations_action(sema.db, def),
            show_fn_references_action(sema.db, def),
            runnable_action(sema, def, file_id),
            goto_type_action_for_def(sema.db, def, &notable_traits),
        ]
        .into_iter()
        .flatten()
        .collect(),
    }
}

fn notable_traits(
    db: &RootDatabase,
    ty: &hir::Type,
) -> Vec<(hir::Trait, Vec<(Option<hir::Type>, hir::Name)>)> {
    db.notable_traits_in_deps(ty.krate(db).into())
        .iter()
        .flat_map(|it| &**it)
        .filter_map(move |&trait_| {
            let trait_ = trait_.into();
            ty.impls_trait(db, trait_, &[]).then(|| {
                (
                    trait_,
                    trait_
                        .items(db)
                        .into_iter()
                        .filter_map(hir::AssocItem::as_type_alias)
                        .map(|alias| {
                            (ty.normalize_trait_assoc_type(db, &[], alias), alias.name(db))
                        })
                        .collect::<Vec<_>>(),
                )
            })
        })
        .collect::<Vec<_>>()
}

fn show_implementations_action(db: &RootDatabase, def: Definition) -> Option<HoverAction> {
    fn to_action(nav_target: NavigationTarget) -> HoverAction {
        HoverAction::Implementation(FilePosition {
            file_id: nav_target.file_id,
            offset: nav_target.focus_or_full_range().start(),
        })
    }

    let adt = match def {
        Definition::Trait(it) => {
            return it.try_to_nav(db).map(UpmappingResult::call_site).map(to_action)
        }
        Definition::Adt(it) => Some(it),
        Definition::SelfType(it) => it.self_ty(db).as_adt(),
        _ => None,
    }?;
    adt.try_to_nav(db).map(UpmappingResult::call_site).map(to_action)
}

fn show_fn_references_action(db: &RootDatabase, def: Definition) -> Option<HoverAction> {
    match def {
        Definition::Function(it) => {
            it.try_to_nav(db).map(UpmappingResult::call_site).map(|nav_target| {
                HoverAction::Reference(FilePosition {
                    file_id: nav_target.file_id,
                    offset: nav_target.focus_or_full_range().start(),
                })
            })
        }
        _ => None,
    }
}

fn runnable_action(
    sema: &hir::Semantics<'_, RootDatabase>,
    def: Definition,
    file_id: FileId,
) -> Option<HoverAction> {
    match def {
        Definition::Module(it) => runnable_mod(sema, it).map(HoverAction::Runnable),
        Definition::Function(func) => {
            let src = func.source(sema.db)?;
            if src.file_id != file_id.into() {
                cov_mark::hit!(hover_macro_generated_struct_fn_doc_comment);
                cov_mark::hit!(hover_macro_generated_struct_fn_doc_attr);
                return None;
            }

            runnable_fn(sema, func).map(HoverAction::Runnable)
        }
        _ => None,
    }
}

fn goto_type_action_for_def(
    db: &RootDatabase,
    def: Definition,
    notable_traits: &[(hir::Trait, Vec<(Option<hir::Type>, hir::Name)>)],
) -> Option<HoverAction> {
    let mut targets: Vec<hir::ModuleDef> = Vec::new();
    let mut push_new_def = |item: hir::ModuleDef| {
        if !targets.contains(&item) {
            targets.push(item);
        }
    };

    for &(trait_, ref assocs) in notable_traits {
        push_new_def(trait_.into());
        assocs.iter().filter_map(|(ty, _)| ty.as_ref()).for_each(|ty| {
            walk_and_push_ty(db, ty, &mut push_new_def);
        });
    }

    if let Definition::GenericParam(hir::GenericParam::TypeParam(it)) = def {
        let krate = it.module(db).krate();
        let sized_trait =
            db.lang_item(krate.into(), LangItem::Sized).and_then(|lang_item| lang_item.as_trait());

        it.trait_bounds(db)
            .into_iter()
            .filter(|&it| Some(it.into()) != sized_trait)
            .for_each(|it| push_new_def(it.into()));
    } else {
        let ty = match def {
            Definition::Local(it) => it.ty(db),
            Definition::GenericParam(hir::GenericParam::ConstParam(it)) => it.ty(db),
            Definition::Field(field) => field.ty(db),
            Definition::Function(function) => function.ret_type(db),
            _ => return HoverAction::goto_type_from_targets(db, targets),
        };

        walk_and_push_ty(db, &ty, &mut push_new_def);
    }

    HoverAction::goto_type_from_targets(db, targets)
}

fn walk_and_push_ty(
    db: &RootDatabase,
    ty: &hir::Type,
    push_new_def: &mut dyn FnMut(hir::ModuleDef),
) {
    ty.walk(db, |t| {
        if let Some(adt) = t.as_adt() {
            push_new_def(adt.into());
        } else if let Some(trait_) = t.as_dyn_trait() {
            push_new_def(trait_.into());
        } else if let Some(traits) = t.as_impl_traits(db) {
            traits.for_each(|it| push_new_def(it.into()));
        } else if let Some(trait_) = t.as_associated_type_parent_trait(db) {
            push_new_def(trait_.into());
        }
    });
}

fn dedupe_or_merge_hover_actions(actions: Vec<HoverAction>) -> Vec<HoverAction> {
    let mut deduped_actions = Vec::with_capacity(actions.len());
    let mut go_to_type_targets = FxIndexSet::default();

    let mut seen_implementation = false;
    let mut seen_reference = false;
    let mut seen_runnable = false;
    for action in actions {
        match action {
            HoverAction::GoToType(targets) => {
                go_to_type_targets.extend(targets);
            }
            HoverAction::Implementation(..) => {
                if !seen_implementation {
                    seen_implementation = true;
                    deduped_actions.push(action);
                }
            }
            HoverAction::Reference(..) => {
                if !seen_reference {
                    seen_reference = true;
                    deduped_actions.push(action);
                }
            }
            HoverAction::Runnable(..) => {
                if !seen_runnable {
                    seen_runnable = true;
                    deduped_actions.push(action);
                }
            }
        };
    }

    if !go_to_type_targets.is_empty() {
        deduped_actions.push(HoverAction::GoToType(
            go_to_type_targets.into_iter().sorted_by(|a, b| a.mod_path.cmp(&b.mod_path)).collect(),
        ));
    }

    deduped_actions
}
