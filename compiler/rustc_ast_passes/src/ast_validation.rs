// Validate AST before lowering it to HIR.
//
// This pass is supposed to catch things that fit into AST data structures,
// but not permitted by the language. It runs after expansion when AST is frozen,
// so it can check for erroneous constructions produced by syntax extensions.
// This pass is supposed to perform only simple checks not requiring name resolution
// or type checking or some other kind of complex analysis.

use itertools::{Either, Itertools};
use rustc_ast::ptr::P;
use rustc_ast::visit::{walk_list, AssocCtxt, BoundKind, FnCtxt, FnKind, Visitor};
use rustc_ast::*;
use rustc_ast_pretty::pprust::{self, State};
use rustc_data_structures::fx::FxIndexMap;
use rustc_feature::Features;
use rustc_parse::validate_attr;
use rustc_session::lint::builtin::{
    DEPRECATED_WHERE_CLAUSE_LOCATION, MISSING_ABI, PATTERNS_IN_FNS_WITHOUT_BODY,
};
use rustc_session::lint::{BuiltinLintDiag, LintBuffer};
use rustc_session::Session;
use rustc_span::symbol::{kw, sym, Ident};
use rustc_span::Span;
use rustc_target::spec::abi;
use std::mem;
use std::ops::{Deref, DerefMut};
use thin_vec::thin_vec;

use crate::errors;
use crate::fluent_generated as fluent;

/// Is `self` allowed semantically as the first parameter in an `FnDecl`?
enum SelfSemantic {
    Yes,
    No,
}

/// What is the context that prevents using `~const`?
// FIXME(effects): Consider getting rid of this in favor of `errors::TildeConstReason`, they're
// almost identical. This gets rid of an abstraction layer which might be considered bad.
enum DisallowTildeConstContext<'a> {
    TraitObject,
    Fn(FnKind<'a>),
    Trait(Span),
    TraitImpl(Span),
    Impl(Span),
    TraitAssocTy(Span),
    TraitImplAssocTy(Span),
    InherentAssocTy(Span),
    Item,
}

enum TraitOrTraitImpl<'a> {
    Trait { span: Span, constness: Option<Span> },
    TraitImpl { constness: Const, polarity: ImplPolarity, trait_ref: &'a TraitRef },
}

impl<'a> TraitOrTraitImpl<'a> {
    fn constness(&self) -> Option<Span> {
        match self {
            Self::Trait { constness: Some(span), .. }
            | Self::TraitImpl { constness: Const::Yes(span), .. } => Some(*span),
            _ => None,
        }
    }
}

struct AstValidator<'a> {
    session: &'a Session,
    features: &'a Features,

    /// The span of the `extern` in an `extern { ... }` block, if any.
    extern_mod: Option<&'a Item>,

    outer_trait_or_trait_impl: Option<TraitOrTraitImpl<'a>>,

    has_proc_macro_decls: bool,

    /// Used to ban nested `impl Trait`, e.g., `impl Into<impl Debug>`.
    /// Nested `impl Trait` _is_ allowed in associated type position,
    /// e.g., `impl Iterator<Item = impl Debug>`.
    outer_impl_trait: Option<Span>,

    disallow_tilde_const: Option<DisallowTildeConstContext<'a>>,

    /// Used to ban `impl Trait` in path projections like `<impl Iterator>::Item`
    /// or `Foo::Bar<impl Trait>`
    is_impl_trait_banned: bool,

    lint_buffer: &'a mut LintBuffer,
}

impl<'a> AstValidator<'a> {
    fn with_in_trait_impl(
        &mut self,
        trait_: Option<(Const, ImplPolarity, &'a TraitRef)>,
        f: impl FnOnce(&mut Self),
    ) {
        let old = mem::replace(
            &mut self.outer_trait_or_trait_impl,
            trait_.map(|(constness, polarity, trait_ref)| TraitOrTraitImpl::TraitImpl {
                constness,
                polarity,
                trait_ref,
            }),
        );
        f(self);
        self.outer_trait_or_trait_impl = old;
    }

    fn with_in_trait(&mut self, span: Span, constness: Option<Span>, f: impl FnOnce(&mut Self)) {
        let old = mem::replace(
            &mut self.outer_trait_or_trait_impl,
            Some(TraitOrTraitImpl::Trait { span, constness }),
        );
        f(self);
        self.outer_trait_or_trait_impl = old;
    }

    fn with_banned_impl_trait(&mut self, f: impl FnOnce(&mut Self)) {
        let old = mem::replace(&mut self.is_impl_trait_banned, true);
        f(self);
        self.is_impl_trait_banned = old;
    }

    fn with_tilde_const(
        &mut self,
        disallowed: Option<DisallowTildeConstContext<'a>>,
        f: impl FnOnce(&mut Self),
    ) {
        let old = mem::replace(&mut self.disallow_tilde_const, disallowed);
        f(self);
        self.disallow_tilde_const = old;
    }

    fn check_type_alias_where_clause_location(
        &mut self,
        ty_alias: &TyAlias,
    ) -> Result<(), errors::WhereClauseBeforeTypeAlias> {
        if ty_alias.ty.is_none() || !ty_alias.where_clauses.before.has_where_token {
            return Ok(());
        }

        let (before_predicates, after_predicates) =
            ty_alias.generics.where_clause.predicates.split_at(ty_alias.where_clauses.split);
        let span = ty_alias.where_clauses.before.span;

        let sugg = if !before_predicates.is_empty() || !ty_alias.where_clauses.after.has_where_token
        {
            let mut state = State::new();

            if !ty_alias.where_clauses.after.has_where_token {
                state.space();
                state.word_space("where");
            }

            let mut first = after_predicates.is_empty();
            for p in before_predicates {
                if !first {
                    state.word_space(",");
                }
                first = false;
                state.print_where_predicate(p);
            }

            errors::WhereClauseBeforeTypeAliasSugg::Move {
                left: span,
                snippet: state.s.eof(),
                right: ty_alias.where_clauses.after.span.shrink_to_hi(),
            }
        } else {
            errors::WhereClauseBeforeTypeAliasSugg::Remove { span }
        };

        Err(errors::WhereClauseBeforeTypeAlias { span, sugg })
    }

    fn with_impl_trait(&mut self, outer: Option<Span>, f: impl FnOnce(&mut Self)) {
        let old = mem::replace(&mut self.outer_impl_trait, outer);
        f(self);
        self.outer_impl_trait = old;
    }

    // Mirrors `visit::walk_ty`, but tracks relevant state.
    fn walk_ty(&mut self, t: &'a Ty) {
        match &t.kind {
            TyKind::ImplTrait(..) => {
                self.with_impl_trait(Some(t.span), |this| visit::walk_ty(this, t))
            }
            TyKind::TraitObject(..) => self
                .with_tilde_const(Some(DisallowTildeConstContext::TraitObject), |this| {
                    visit::walk_ty(this, t)
                }),
            TyKind::Path(qself, path) => {
                // We allow these:
                //  - `Option<impl Trait>`
                //  - `option::Option<impl Trait>`
                //  - `option::Option<T>::Foo<impl Trait>`
                //
                // But not these:
                //  - `<impl Trait>::Foo`
                //  - `option::Option<impl Trait>::Foo`.
                //
                // To implement this, we disallow `impl Trait` from `qself`
                // (for cases like `<impl Trait>::Foo>`)
                // but we allow `impl Trait` in `GenericArgs`
                // iff there are no more PathSegments.
                if let Some(qself) = qself {
                    // `impl Trait` in `qself` is always illegal
                    self.with_banned_impl_trait(|this| this.visit_ty(&qself.ty));
                }

                // Note that there should be a call to visit_path here,
                // so if any logic is added to process `Path`s a call to it should be
                // added both in visit_path and here. This code mirrors visit::walk_path.
                for (i, segment) in path.segments.iter().enumerate() {
                    // Allow `impl Trait` iff we're on the final path segment
                    if i == path.segments.len() - 1 {
                        self.visit_path_segment(segment);
                    } else {
                        self.with_banned_impl_trait(|this| this.visit_path_segment(segment));
                    }
                }
            }
            TyKind::AnonStruct(_, ref fields) | TyKind::AnonUnion(_, ref fields) => {
                walk_list!(self, visit_struct_field_def, fields)
            }
            _ => visit::walk_ty(self, t),
        }
    }

    fn visit_struct_field_def(&mut self, field: &'a FieldDef) {
        if let Some(ident) = field.ident
            && ident.name == kw::Underscore
        {
            self.check_unnamed_field_ty(&field.ty, ident.span);
            self.visit_vis(&field.vis);
            self.visit_ident(ident);
            self.visit_ty_common(&field.ty);
            self.walk_ty(&field.ty);
            walk_list!(self, visit_attribute, &field.attrs);
        } else {
            self.visit_field_def(field);
        }
    }

    fn dcx(&self) -> &rustc_errors::DiagCtxt {
        self.session.dcx()
    }

    fn check_lifetime(&self, ident: Ident) {
        let valid_names = [kw::UnderscoreLifetime, kw::StaticLifetime, kw::Empty];
        if !valid_names.contains(&ident.name) && ident.without_first_quote().is_reserved() {
            self.dcx().emit_err(errors::KeywordLifetime { span: ident.span });
        }
    }

    fn check_label(&self, ident: Ident) {
        if ident.without_first_quote().is_reserved() {
            self.dcx().emit_err(errors::InvalidLabel { span: ident.span, name: ident.name });
        }
    }

    fn visibility_not_permitted(&self, vis: &Visibility, note: errors::VisibilityNotPermittedNote) {
        if let VisibilityKind::Inherited = vis.kind {
            return;
        }

        self.dcx().emit_err(errors::VisibilityNotPermitted { span: vis.span, note });
    }

    fn check_decl_no_pat(decl: &FnDecl, mut report_err: impl FnMut(Span, Option<Ident>, bool)) {
        for Param { pat, .. } in &decl.inputs {
            match pat.kind {
                PatKind::Ident(BindingAnnotation::NONE, _, None) | PatKind::Wild => {}
                PatKind::Ident(BindingAnnotation::MUT, ident, None) => {
                    report_err(pat.span, Some(ident), true)
                }
                _ => report_err(pat.span, None, false),
            }
        }
    }

    fn check_unnamed_field_ty(&self, ty: &Ty, span: Span) {
        if matches!(
            &ty.kind,
            // We already checked for `kw::Underscore` before calling this function,
            // so skip the check
            TyKind::AnonStruct(..) | TyKind::AnonUnion(..)
            // If the anonymous field contains a Path as type, we can't determine
            // if the path is a valid struct or union, so skip the check
            | TyKind::Path(..)
        ) {
            return;
        }
        self.dcx().emit_err(errors::InvalidUnnamedFieldTy { span, ty_span: ty.span });
    }

    fn deny_anon_struct_or_union(&self, ty: &Ty) {
        let struct_or_union = match &ty.kind {
            TyKind::AnonStruct(..) => "struct",
            TyKind::AnonUnion(..) => "union",
            _ => return,
        };
        self.dcx().emit_err(errors::AnonStructOrUnionNotAllowed { struct_or_union, span: ty.span });
    }

    fn deny_unnamed_field(&self, field: &FieldDef) {
        if let Some(ident) = field.ident
            && ident.name == kw::Underscore
        {
            self.dcx()
                .emit_err(errors::InvalidUnnamedField { span: field.span, ident_span: ident.span });
        }
    }

    fn check_trait_fn_not_const(&self, constness: Const, parent: &TraitOrTraitImpl<'a>) {
        let Const::Yes(span) = constness else {
            return;
        };

        let make_impl_const_sugg = if self.features.const_trait_impl
            && let TraitOrTraitImpl::TraitImpl {
                constness: Const::No,
                polarity: ImplPolarity::Positive,
                trait_ref,
                ..
            } = parent
        {
            Some(trait_ref.path.span.shrink_to_lo())
        } else {
            None
        };

        let make_trait_const_sugg = if self.features.const_trait_impl
            && let TraitOrTraitImpl::Trait { span, constness: None } = parent
        {
            Some(span.shrink_to_lo())
        } else {
            None
        };

        let parent_constness = parent.constness();
        self.dcx().emit_err(errors::TraitFnConst {
            span,
            in_impl: matches!(parent, TraitOrTraitImpl::TraitImpl { .. }),
            const_context_label: parent_constness,
            remove_const_sugg: (
                self.session.source_map().span_extend_while(span, |c| c == ' ').unwrap_or(span),
                match parent_constness {
                    Some(_) => rustc_errors::Applicability::MachineApplicable,
                    None => rustc_errors::Applicability::MaybeIncorrect,
                },
            ),
            requires_multiple_changes: make_impl_const_sugg.is_some()
                || make_trait_const_sugg.is_some(),
            make_impl_const_sugg,
            make_trait_const_sugg,
        });
    }

    fn check_fn_decl(&self, fn_decl: &FnDecl, self_semantic: SelfSemantic) {
        self.check_decl_num_args(fn_decl);
        self.check_decl_cvaradic_pos(fn_decl);
        self.check_decl_attrs(fn_decl);
        self.check_decl_self_param(fn_decl, self_semantic);
    }

    /// Emits fatal error if function declaration has more than `u16::MAX` arguments
    /// Error is fatal to prevent errors during typechecking
    fn check_decl_num_args(&self, fn_decl: &FnDecl) {
        let max_num_args: usize = u16::MAX.into();
        if fn_decl.inputs.len() > max_num_args {
            let Param { span, .. } = fn_decl.inputs[0];
            self.dcx().emit_fatal(errors::FnParamTooMany { span, max_num_args });
        }
    }

    fn check_decl_cvaradic_pos(&self, fn_decl: &FnDecl) {
        match &*fn_decl.inputs {
            [Param { ty, span, .. }] => {
                if let TyKind::CVarArgs = ty.kind {
                    self.dcx().emit_err(errors::FnParamCVarArgsOnly { span: *span });
                }
            }
            [ps @ .., _] => {
                for Param { ty, span, .. } in ps {
                    if let TyKind::CVarArgs = ty.kind {
                        self.dcx().emit_err(errors::FnParamCVarArgsNotLast { span: *span });
                    }
                }
            }
            _ => {}
        }
    }

    fn check_decl_attrs(&self, fn_decl: &FnDecl) {
        fn_decl
            .inputs
            .iter()
            .flat_map(|i| i.attrs.as_ref())
            .filter(|attr| {
                let arr = [
                    sym::allow,
                    sym::cfg,
                    sym::cfg_attr,
                    sym::deny,
                    sym::expect,
                    sym::forbid,
                    sym::warn,
                ];
                !arr.contains(&attr.name_or_empty()) && rustc_attr::is_builtin_attr(attr)
            })
            .for_each(|attr| {
                if attr.is_doc_comment() {
                    self.dcx().emit_err(errors::FnParamDocComment { span: attr.span });
                } else {
                    self.dcx().emit_err(errors::FnParamForbiddenAttr { span: attr.span });
                }
            });
    }

    fn check_decl_self_param(&self, fn_decl: &FnDecl, self_semantic: SelfSemantic) {
        if let (SelfSemantic::No, [param, ..]) = (self_semantic, &*fn_decl.inputs) {
            if param.is_self() {
                self.dcx().emit_err(errors::FnParamForbiddenSelf { span: param.span });
            }
        }
    }

    fn check_defaultness(&self, span: Span, defaultness: Defaultness) {
        if let Defaultness::Default(def_span) = defaultness {
            let span = self.session.source_map().guess_head_span(span);
            self.dcx().emit_err(errors::ForbiddenDefault { span, def_span });
        }
    }

    /// If `sp` ends with a semicolon, returns it as a `Span`
    /// Otherwise, returns `sp.shrink_to_hi()`
    fn ending_semi_or_hi(&self, sp: Span) -> Span {
        let source_map = self.session.source_map();
        let end = source_map.end_point(sp);

        if source_map.span_to_snippet(end).is_ok_and(|s| s == ";") {
            end
        } else {
            sp.shrink_to_hi()
        }
    }

    fn check_type_no_bounds(&self, bounds: &[GenericBound], ctx: &str) {
        let span = match bounds {
            [] => return,
            [b0] => b0.span(),
            [b0, .., bl] => b0.span().to(bl.span()),
        };
        self.dcx().emit_err(errors::BoundInContext { span, ctx });
    }

    fn check_foreign_ty_genericless(
        &self,
        generics: &Generics,
        where_clauses: &TyAliasWhereClauses,
    ) {
        let cannot_have = |span, descr, remove_descr| {
            self.dcx().emit_err(errors::ExternTypesCannotHave {
                span,
                descr,
                remove_descr,
                block_span: self.current_extern_span(),
            });
        };

        if !generics.params.is_empty() {
            cannot_have(generics.span, "generic parameters", "generic parameters");
        }

        let check_where_clause = |where_clause: TyAliasWhereClause| {
            if where_clause.has_where_token {
                cannot_have(where_clause.span, "`where` clauses", "`where` clause");
            }
        };

        check_where_clause(where_clauses.before);
        check_where_clause(where_clauses.after);
    }

    fn check_foreign_kind_bodyless(&self, ident: Ident, kind: &str, body: Option<Span>) {
        let Some(body) = body else {
            return;
        };
        self.dcx().emit_err(errors::BodyInExtern {
            span: ident.span,
            body,
            block: self.current_extern_span(),
            kind,
        });
    }

    /// An `fn` in `extern { ... }` cannot have a body `{ ... }`.
    fn check_foreign_fn_bodyless(&self, ident: Ident, body: Option<&Block>) {
        let Some(body) = body else {
            return;
        };
        self.dcx().emit_err(errors::FnBodyInExtern {
            span: ident.span,
            body: body.span,
            block: self.current_extern_span(),
        });
    }

    fn current_extern_span(&self) -> Span {
        self.session.source_map().guess_head_span(self.extern_mod.unwrap().span)
    }

    /// An `fn` in `extern { ... }` cannot have qualifiers, e.g. `async fn`.
    fn check_foreign_fn_headerless(&self, ident: Ident, span: Span, header: FnHeader) {
        if header.has_qualifiers() {
            self.dcx().emit_err(errors::FnQualifierInExtern {
                span: ident.span,
                block: self.current_extern_span(),
                sugg_span: span.until(ident.span.shrink_to_lo()),
            });
        }
    }

    /// An item in `extern { ... }` cannot use non-ascii identifier.
    fn check_foreign_item_ascii_only(&self, ident: Ident) {
        if !ident.as_str().is_ascii() {
            self.dcx().emit_err(errors::ExternItemAscii {
                span: ident.span,
                block: self.current_extern_span(),
            });
        }
    }

    /// Reject invalid C-variadic types.
    ///
    /// C-variadics must be:
    /// - Non-const
    /// - Either foreign, or free and `unsafe extern "C"` semantically
    fn check_c_variadic_type(&self, fk: FnKind<'a>) {
        let variadic_spans: Vec<_> = fk
            .decl()
            .inputs
            .iter()
            .filter(|arg| matches!(arg.ty.kind, TyKind::CVarArgs))
            .map(|arg| arg.span)
            .collect();

        if variadic_spans.is_empty() {
            return;
        }

        if let Some(header) = fk.header() {
            if let Const::Yes(const_span) = header.constness {
                let mut spans = variadic_spans.clone();
                spans.push(const_span);
                self.dcx().emit_err(errors::ConstAndCVariadic {
                    spans,
                    const_span,
                    variadic_spans: variadic_spans.clone(),
                });
            }
        }

        match (fk.ctxt(), fk.header()) {
            (Some(FnCtxt::Foreign), _) => return,
            (Some(FnCtxt::Free), Some(header)) => match header.ext {
                Extern::Explicit(StrLit { symbol_unescaped: sym::C, .. }, _)
                | Extern::Implicit(_)
                    if matches!(header.unsafety, Unsafe::Yes(_)) =>
                {
                    return;
                }
                _ => {}
            },
            _ => {}
        };

        self.dcx().emit_err(errors::BadCVariadic { span: variadic_spans });
    }

    fn check_item_named(&self, ident: Ident, kind: &str) {
        if ident.name != kw::Underscore {
            return;
        }
        self.dcx().emit_err(errors::ItemUnderscore { span: ident.span, kind });
    }

    fn check_nomangle_item_asciionly(&self, ident: Ident, item_span: Span) {
        if ident.name.as_str().is_ascii() {
            return;
        }
        let span = self.session.source_map().guess_head_span(item_span);
        self.dcx().emit_err(errors::NoMangleAscii { span });
    }

    fn check_mod_file_item_asciionly(&self, ident: Ident) {
        if ident.name.as_str().is_ascii() {
            return;
        }
        self.dcx().emit_err(errors::ModuleNonAscii { span: ident.span, name: ident.name });
    }

    fn deny_generic_params(&self, generics: &Generics, ident: Span) {
        if !generics.params.is_empty() {
            self.dcx().emit_err(errors::AutoTraitGeneric { span: generics.span, ident });
        }
    }

    fn emit_e0568(&self, span: Span, ident: Span) {
        self.dcx().emit_err(errors::AutoTraitBounds { span, ident });
    }

    fn deny_super_traits(&self, bounds: &GenericBounds, ident_span: Span) {
        if let [.., last] = &bounds[..] {
            let span = ident_span.shrink_to_hi().to(last.span());
            self.emit_e0568(span, ident_span);
        }
    }

    fn deny_where_clause(&self, where_clause: &WhereClause, ident_span: Span) {
        if !where_clause.predicates.is_empty() {
            self.emit_e0568(where_clause.span, ident_span);
        }
    }

    fn deny_items(&self, trait_items: &[P<AssocItem>], ident: Span) {
        if !trait_items.is_empty() {
            let spans: Vec<_> = trait_items.iter().map(|i| i.ident.span).collect();
            let total = trait_items.first().unwrap().span.to(trait_items.last().unwrap().span);
            self.dcx().emit_err(errors::AutoTraitItems { spans, total, ident });
        }
    }

    fn correct_generic_order_suggestion(&self, data: &AngleBracketedArgs) -> String {
        // Lifetimes always come first.
        let lt_sugg = data.args.iter().filter_map(|arg| match arg {
            AngleBracketedArg::Arg(lt @ GenericArg::Lifetime(_)) => {
                Some(pprust::to_string(|s| s.print_generic_arg(lt)))
            }
            _ => None,
        });
        let args_sugg = data.args.iter().filter_map(|a| match a {
            AngleBracketedArg::Arg(GenericArg::Lifetime(_)) | AngleBracketedArg::Constraint(_) => {
                None
            }
            AngleBracketedArg::Arg(arg) => Some(pprust::to_string(|s| s.print_generic_arg(arg))),
        });
        // Constraints always come last.
        let constraint_sugg = data.args.iter().filter_map(|a| match a {
            AngleBracketedArg::Arg(_) => None,
            AngleBracketedArg::Constraint(c) => {
                Some(pprust::to_string(|s| s.print_assoc_constraint(c)))
            }
        });
        format!(
            "<{}>",
            lt_sugg.chain(args_sugg).chain(constraint_sugg).collect::<Vec<String>>().join(", ")
        )
    }

    /// Enforce generic args coming before constraints in `<...>` of a path segment.
    fn check_generic_args_before_constraints(&self, data: &AngleBracketedArgs) {
        // Early exit in case it's partitioned as it should be.
        if data.args.iter().is_partitioned(|arg| matches!(arg, AngleBracketedArg::Arg(_))) {
            return;
        }
        // Find all generic argument coming after the first constraint...
        let (constraint_spans, arg_spans): (Vec<Span>, Vec<Span>) =
            data.args.iter().partition_map(|arg| match arg {
                AngleBracketedArg::Constraint(c) => Either::Left(c.span),
                AngleBracketedArg::Arg(a) => Either::Right(a.span()),
            });
        let args_len = arg_spans.len();
        let constraint_len = constraint_spans.len();
        // ...and then error:
        self.dcx().emit_err(errors::ArgsBeforeConstraint {
            arg_spans: arg_spans.clone(),
            constraints: constraint_spans[0],
            args: *arg_spans.iter().last().unwrap(),
            data: data.span,
            constraint_spans: errors::EmptyLabelManySpans(constraint_spans),
            arg_spans2: errors::EmptyLabelManySpans(arg_spans),
            suggestion: self.correct_generic_order_suggestion(data),
            constraint_len,
            args_len,
        });
    }

    fn visit_ty_common(&mut self, ty: &'a Ty) {
        match &ty.kind {
            TyKind::BareFn(bfty) => {
                self.check_fn_decl(&bfty.decl, SelfSemantic::No);
                Self::check_decl_no_pat(&bfty.decl, |span, _, _| {
                    self.dcx().emit_err(errors::PatternFnPointer { span });
                });
                if let Extern::Implicit(_) = bfty.ext {
                    let sig_span = self.session.source_map().next_point(ty.span.shrink_to_lo());
                    self.maybe_lint_missing_abi(sig_span, ty.id);
                }
            }
            TyKind::TraitObject(bounds, ..) => {
                let mut any_lifetime_bounds = false;
                for bound in bounds {
                    if let GenericBound::Outlives(lifetime) = bound {
                        if any_lifetime_bounds {
                            self.dcx()
                                .emit_err(errors::TraitObjectBound { span: lifetime.ident.span });
                            break;
                        }
                        any_lifetime_bounds = true;
                    }
                }
            }
            TyKind::ImplTrait(_, bounds) => {
                if self.is_impl_trait_banned {
                    self.dcx().emit_err(errors::ImplTraitPath { span: ty.span });
                }

                if let Some(outer_impl_trait_sp) = self.outer_impl_trait {
                    self.dcx().emit_err(errors::NestedImplTrait {
                        span: ty.span,
                        outer: outer_impl_trait_sp,
                        inner: ty.span,
                    });
                }

                if !bounds.iter().any(|b| matches!(b, GenericBound::Trait(..))) {
                    self.dcx().emit_err(errors::AtLeastOneTrait { span: ty.span });
                }
            }
            _ => {}
        }
    }

    fn maybe_lint_missing_abi(&mut self, span: Span, id: NodeId) {
        // FIXME(davidtwco): This is a hack to detect macros which produce spans of the
        // call site which do not have a macro backtrace. See #61963.
        if self
            .session
            .source_map()
            .span_to_snippet(span)
            .is_ok_and(|snippet| !snippet.starts_with("#["))
        {
            self.lint_buffer.buffer_lint_with_diagnostic(
                MISSING_ABI,
                id,
                span,
                fluent::ast_passes_extern_without_abi,
                BuiltinLintDiag::MissingAbi(span, abi::Abi::FALLBACK),
            )
        }
    }
}

/// Checks that generic parameters are in the correct order,
/// which is lifetimes, then types and then consts. (`<'a, T, const N: usize>`)
fn validate_generic_param_order(
    dcx: &rustc_errors::DiagCtxt,
    generics: &[GenericParam],
    span: Span,
) {
    let mut max_param: Option<ParamKindOrd> = None;
    let mut out_of_order = FxIndexMap::default();
    let mut param_idents = Vec::with_capacity(generics.len());

    for (idx, param) in generics.iter().enumerate() {
        let ident = param.ident;
        let (kind, bounds, span) = (&param.kind, &param.bounds, ident.span);
        let (ord_kind, ident) = match &param.kind {
            GenericParamKind::Lifetime => (ParamKindOrd::Lifetime, ident.to_string()),
            GenericParamKind::Type { .. } => (ParamKindOrd::TypeOrConst, ident.to_string()),
            GenericParamKind::Const { ty, .. } => {
                let ty = pprust::ty_to_string(ty);
                (ParamKindOrd::TypeOrConst, format!("const {ident}: {ty}"))
            }
        };
        param_idents.push((kind, ord_kind, bounds, idx, ident));
        match max_param {
            Some(max_param) if max_param > ord_kind => {
                let entry = out_of_order.entry(ord_kind).or_insert((max_param, vec![]));
                entry.1.push(span);
            }
            Some(_) | None => max_param = Some(ord_kind),
        };
    }

    if !out_of_order.is_empty() {
        let mut ordered_params = "<".to_string();
        param_idents.sort_by_key(|&(_, po, _, i, _)| (po, i));
        let mut first = true;
        for (kind, _, bounds, _, ident) in param_idents {
            if !first {
                ordered_params += ", ";
            }
            ordered_params += &ident;

            if !bounds.is_empty() {
                ordered_params += ": ";
                ordered_params += &pprust::bounds_to_string(bounds);
            }

            match kind {
                GenericParamKind::Type { default: Some(default) } => {
                    ordered_params += " = ";
                    ordered_params += &pprust::ty_to_string(default);
                }
                GenericParamKind::Type { default: None } => (),
                GenericParamKind::Lifetime => (),
                GenericParamKind::Const { ty: _, kw_span: _, default: Some(default) } => {
                    ordered_params += " = ";
                    ordered_params += &pprust::expr_to_string(&default.value);
                }
                GenericParamKind::Const { ty: _, kw_span: _, default: None } => (),
            }
            first = false;
        }

        ordered_params += ">";

        for (param_ord, (max_param, spans)) in &out_of_order {
            dcx.emit_err(errors::OutOfOrderParams {
                spans: spans.clone(),
                sugg_span: span,
                param_ord,
                max_param,
                ordered_params: &ordered_params,
            });
        }
    }
}

impl<'a> Visitor<'a> for AstValidator<'a> {
    fn visit_attribute(&mut self, attr: &Attribute) {
        validate_attr::check_attr(&self.session.psess, attr);
    }

    fn visit_ty(&mut self, ty: &'a Ty) {
        self.visit_ty_common(ty);
        self.deny_anon_struct_or_union(ty);
        self.walk_ty(ty)
    }

    fn visit_label(&mut self, label: &'a Label) {
        self.check_label(label.ident);
        visit::walk_label(self, label);
    }

    fn visit_lifetime(&mut self, lifetime: &'a Lifetime, _: visit::LifetimeCtxt) {
        self.check_lifetime(lifetime.ident);
        visit::walk_lifetime(self, lifetime);
    }

    fn visit_field_def(&mut self, field: &'a FieldDef) {
        self.deny_unnamed_field(field);
        visit::walk_field_def(self, field)
    }

    fn visit_item(&mut self, item: &'a Item) {
        if item.attrs.iter().any(|attr| attr.is_proc_macro_attr()) {
            self.has_proc_macro_decls = true;
        }

        if attr::contains_name(&item.attrs, sym::no_mangle) {
            self.check_nomangle_item_asciionly(item.ident, item.span);
        }

        match &item.kind {
            ItemKind::Impl(box Impl {
                unsafety,
                polarity,
                defaultness: _,
                constness,
                generics,
                of_trait: Some(t),
                self_ty,
                items,
            }) => {
                self.with_in_trait_impl(Some((*constness, *polarity, t)), |this| {
                    this.visibility_not_permitted(
                        &item.vis,
                        errors::VisibilityNotPermittedNote::TraitImpl,
                    );
                    if let TyKind::Dummy = self_ty.kind {
                        // Abort immediately otherwise the `TyKind::Dummy` will reach HIR lowering,
                        // which isn't allowed. Not a problem for this obscure, obsolete syntax.
                        this.dcx().emit_fatal(errors::ObsoleteAuto { span: item.span });
                    }
                    if let (&Unsafe::Yes(span), &ImplPolarity::Negative(sp)) = (unsafety, polarity)
                    {
                        this.dcx().emit_err(errors::UnsafeNegativeImpl {
                            span: sp.to(t.path.span),
                            negative: sp,
                            r#unsafe: span,
                        });
                    }

                    this.visit_vis(&item.vis);
                    this.visit_ident(item.ident);
                    let disallowed = matches!(constness, Const::No)
                        .then(|| DisallowTildeConstContext::TraitImpl(item.span));
                    this.with_tilde_const(disallowed, |this| this.visit_generics(generics));
                    this.visit_trait_ref(t);
                    this.visit_ty(self_ty);

                    walk_list!(this, visit_assoc_item, items, AssocCtxt::Impl);
                });
                walk_list!(self, visit_attribute, &item.attrs);
                return; // Avoid visiting again.
            }
            ItemKind::Impl(box Impl {
                unsafety,
                polarity,
                defaultness,
                constness,
                generics,
                of_trait: None,
                self_ty,
                items,
            }) => {
                let error =
                    |annotation_span, annotation, only_trait: bool| errors::InherentImplCannot {
                        span: self_ty.span,
                        annotation_span,
                        annotation,
                        self_ty: self_ty.span,
                        only_trait: only_trait.then_some(()),
                    };

                self.with_in_trait_impl(None, |this| {
                    this.visibility_not_permitted(
                        &item.vis,
                        errors::VisibilityNotPermittedNote::IndividualImplItems,
                    );
                    if let &Unsafe::Yes(span) = unsafety {
                        this.dcx().emit_err(errors::InherentImplCannotUnsafe {
                            span: self_ty.span,
                            annotation_span: span,
                            annotation: "unsafe",
                            self_ty: self_ty.span,
                        });
                    }
                    if let &ImplPolarity::Negative(span) = polarity {
                        this.dcx().emit_err(error(span, "negative", false));
                    }
                    if let &Defaultness::Default(def_span) = defaultness {
                        this.dcx().emit_err(error(def_span, "`default`", true));
                    }
                    if let &Const::Yes(span) = constness {
                        this.dcx().emit_err(error(span, "`const`", true));
                    }

                    this.visit_vis(&item.vis);
                    this.visit_ident(item.ident);
                    this.with_tilde_const(
                        Some(DisallowTildeConstContext::Impl(item.span)),
                        |this| this.visit_generics(generics),
                    );
                    this.visit_ty(self_ty);
                    walk_list!(this, visit_assoc_item, items, AssocCtxt::Impl);
                });
                walk_list!(self, visit_attribute, &item.attrs);
                return; // Avoid visiting again.
            }
            ItemKind::Fn(box Fn { defaultness, sig, generics, body }) => {
                self.check_defaultness(item.span, *defaultness);

                if body.is_none() {
                    self.dcx().emit_err(errors::FnWithoutBody {
                        span: item.span,
                        replace_span: self.ending_semi_or_hi(item.span),
                        extern_block_suggestion: match sig.header.ext {
                            Extern::None => None,
                            Extern::Implicit(start_span) => {
                                Some(errors::ExternBlockSuggestion::Implicit {
                                    start_span,
                                    end_span: item.span.shrink_to_hi(),
                                })
                            }
                            Extern::Explicit(abi, start_span) => {
                                Some(errors::ExternBlockSuggestion::Explicit {
                                    start_span,
                                    end_span: item.span.shrink_to_hi(),
                                    abi: abi.symbol_unescaped,
                                })
                            }
                        },
                    });
                }

                self.visit_vis(&item.vis);
                self.visit_ident(item.ident);
                let kind =
                    FnKind::Fn(FnCtxt::Free, item.ident, sig, &item.vis, generics, body.as_deref());
                self.visit_fn(kind, item.span, item.id);
                walk_list!(self, visit_attribute, &item.attrs);
                return; // Avoid visiting again.
            }
            ItemKind::ForeignMod(ForeignMod { abi, unsafety, .. }) => {
                let old_item = mem::replace(&mut self.extern_mod, Some(item));
                self.visibility_not_permitted(
                    &item.vis,
                    errors::VisibilityNotPermittedNote::IndividualForeignItems,
                );
                if let &Unsafe::Yes(span) = unsafety {
                    self.dcx().emit_err(errors::UnsafeItem { span, kind: "extern block" });
                }
                if abi.is_none() {
                    self.maybe_lint_missing_abi(item.span, item.id);
                }
                visit::walk_item(self, item);
                self.extern_mod = old_item;
                return; // Avoid visiting again.
            }
            ItemKind::Enum(def, _) => {
                for variant in &def.variants {
                    self.visibility_not_permitted(
                        &variant.vis,
                        errors::VisibilityNotPermittedNote::EnumVariant,
                    );
                    for field in variant.data.fields() {
                        self.visibility_not_permitted(
                            &field.vis,
                            errors::VisibilityNotPermittedNote::EnumVariant,
                        );
                    }
                }
            }
            ItemKind::Trait(box Trait { is_auto, generics, bounds, items, .. }) => {
                let is_const_trait =
                    attr::find_by_name(&item.attrs, sym::const_trait).map(|attr| attr.span);
                self.with_in_trait(item.span, is_const_trait, |this| {
                    if *is_auto == IsAuto::Yes {
                        // Auto traits cannot have generics, super traits nor contain items.
                        this.deny_generic_params(generics, item.ident.span);
                        this.deny_super_traits(bounds, item.ident.span);
                        this.deny_where_clause(&generics.where_clause, item.ident.span);
                        this.deny_items(items, item.ident.span);
                    }

                    // Equivalent of `visit::walk_item` for `ItemKind::Trait` that inserts a bound
                    // context for the supertraits.
                    this.visit_vis(&item.vis);
                    this.visit_ident(item.ident);
                    let disallowed = is_const_trait
                        .is_none()
                        .then(|| DisallowTildeConstContext::Trait(item.span));
                    this.with_tilde_const(disallowed, |this| {
                        this.visit_generics(generics);
                        walk_list!(this, visit_param_bound, bounds, BoundKind::SuperTraits)
                    });
                    walk_list!(this, visit_assoc_item, items, AssocCtxt::Trait);
                });
                walk_list!(self, visit_attribute, &item.attrs);
                return; // Avoid visiting again
            }
            ItemKind::Mod(unsafety, mod_kind) => {
                if let &Unsafe::Yes(span) = unsafety {
                    self.dcx().emit_err(errors::UnsafeItem { span, kind: "module" });
                }
                // Ensure that `path` attributes on modules are recorded as used (cf. issue #35584).
                if !matches!(mod_kind, ModKind::Loaded(_, Inline::Yes, _))
                    && !attr::contains_name(&item.attrs, sym::path)
                {
                    self.check_mod_file_item_asciionly(item.ident);
                }
            }
            ItemKind::Struct(vdata, generics) => match vdata {
                VariantData::Struct { fields, .. } => {
                    self.visit_vis(&item.vis);
                    self.visit_ident(item.ident);
                    self.visit_generics(generics);
                    // Permit `Anon{Struct,Union}` as field type.
                    walk_list!(self, visit_struct_field_def, fields);
                    walk_list!(self, visit_attribute, &item.attrs);
                    return;
                }
                _ => {}
            },
            ItemKind::Union(vdata, generics) => {
                if vdata.fields().is_empty() {
                    self.dcx().emit_err(errors::FieldlessUnion { span: item.span });
                }
                match vdata {
                    VariantData::Struct { fields, .. } => {
                        self.visit_vis(&item.vis);
                        self.visit_ident(item.ident);
                        self.visit_generics(generics);
                        // Permit `Anon{Struct,Union}` as field type.
                        walk_list!(self, visit_struct_field_def, fields);
                        walk_list!(self, visit_attribute, &item.attrs);
                        return;
                    }
                    _ => {}
                }
            }
            ItemKind::Const(box ConstItem { defaultness, expr, .. }) => {
                self.check_defaultness(item.span, *defaultness);
                if expr.is_none() {
                    self.dcx().emit_err(errors::ConstWithoutBody {
                        span: item.span,
                        replace_span: self.ending_semi_or_hi(item.span),
                    });
                }
            }
            ItemKind::Static(box StaticItem { expr: None, .. }) => {
                self.dcx().emit_err(errors::StaticWithoutBody {
                    span: item.span,
                    replace_span: self.ending_semi_or_hi(item.span),
                });
            }
            ItemKind::TyAlias(
                ty_alias @ box TyAlias { defaultness, bounds, where_clauses, ty, .. },
            ) => {
                self.check_defaultness(item.span, *defaultness);
                if ty.is_none() {
                    self.dcx().emit_err(errors::TyAliasWithoutBody {
                        span: item.span,
                        replace_span: self.ending_semi_or_hi(item.span),
                    });
                }
                self.check_type_no_bounds(bounds, "this context");

                if self.features.lazy_type_alias {
                    if let Err(err) = self.check_type_alias_where_clause_location(ty_alias) {
                        self.dcx().emit_err(err);
                    }
                } else if where_clauses.after.has_where_token {
                    self.dcx().emit_err(errors::WhereClauseAfterTypeAlias {
                        span: where_clauses.after.span,
                        help: self.session.is_nightly_build().then_some(()),
                    });
                }
            }
            _ => {}
        }

        visit::walk_item(self, item);
    }

    fn visit_foreign_item(&mut self, fi: &'a ForeignItem) {
        match &fi.kind {
            ForeignItemKind::Fn(box Fn { defaultness, sig, body, .. }) => {
                self.check_defaultness(fi.span, *defaultness);
                self.check_foreign_fn_bodyless(fi.ident, body.as_deref());
                self.check_foreign_fn_headerless(fi.ident, fi.span, sig.header);
                self.check_foreign_item_ascii_only(fi.ident);
            }
            ForeignItemKind::TyAlias(box TyAlias {
                defaultness,
                generics,
                where_clauses,
                bounds,
                ty,
                ..
            }) => {
                self.check_defaultness(fi.span, *defaultness);
                self.check_foreign_kind_bodyless(fi.ident, "type", ty.as_ref().map(|b| b.span));
                self.check_type_no_bounds(bounds, "`extern` blocks");
                self.check_foreign_ty_genericless(generics, where_clauses);
                self.check_foreign_item_ascii_only(fi.ident);
            }
            ForeignItemKind::Static(_, _, body) => {
                self.check_foreign_kind_bodyless(fi.ident, "static", body.as_ref().map(|b| b.span));
                self.check_foreign_item_ascii_only(fi.ident);
            }
            ForeignItemKind::MacCall(..) => {}
        }

        visit::walk_foreign_item(self, fi)
    }

    // Mirrors `visit::walk_generic_args`, but tracks relevant state.
    fn visit_generic_args(&mut self, generic_args: &'a GenericArgs) {
        match generic_args {
            GenericArgs::AngleBracketed(data) => {
                self.check_generic_args_before_constraints(data);

                for arg in &data.args {
                    match arg {
                        AngleBracketedArg::Arg(arg) => self.visit_generic_arg(arg),
                        // Type bindings such as `Item = impl Debug` in `Iterator<Item = Debug>`
                        // are allowed to contain nested `impl Trait`.
                        AngleBracketedArg::Constraint(constraint) => {
                            self.with_impl_trait(None, |this| {
                                this.visit_assoc_constraint(constraint);
                            });
                        }
                    }
                }
            }
            GenericArgs::Parenthesized(data) => {
                walk_list!(self, visit_ty, &data.inputs);
                if let FnRetTy::Ty(ty) = &data.output {
                    // `-> Foo` syntax is essentially an associated type binding,
                    // so it is also allowed to contain nested `impl Trait`.
                    self.with_impl_trait(None, |this| this.visit_ty(ty));
                }
            }
        }
    }

    fn visit_generics(&mut self, generics: &'a Generics) {
        let mut prev_param_default = None;
        for param in &generics.params {
            match param.kind {
                GenericParamKind::Lifetime => (),
                GenericParamKind::Type { default: Some(_), .. }
                | GenericParamKind::Const { default: Some(_), .. } => {
                    prev_param_default = Some(param.ident.span);
                }
                GenericParamKind::Type { .. } | GenericParamKind::Const { .. } => {
                    if let Some(span) = prev_param_default {
                        self.dcx().emit_err(errors::GenericDefaultTrailing { span });
                        break;
                    }
                }
            }
        }

        validate_generic_param_order(self.dcx(), &generics.params, generics.span);

        for predicate in &generics.where_clause.predicates {
            if let WherePredicate::EqPredicate(predicate) = predicate {
                deny_equality_constraints(self, predicate, generics);
            }
        }
        walk_list!(self, visit_generic_param, &generics.params);
        for predicate in &generics.where_clause.predicates {
            match predicate {
                WherePredicate::BoundPredicate(bound_pred) => {
                    // This is slightly complicated. Our representation for poly-trait-refs contains a single
                    // binder and thus we only allow a single level of quantification. However,
                    // the syntax of Rust permits quantification in two places in where clauses,
                    // e.g., `T: for <'a> Foo<'a>` and `for <'a, 'b> &'b T: Foo<'a>`. If both are
                    // defined, then error.
                    if !bound_pred.bound_generic_params.is_empty() {
                        for bound in &bound_pred.bounds {
                            match bound {
                                GenericBound::Trait(t, _) => {
                                    if !t.bound_generic_params.is_empty() {
                                        self.dcx()
                                            .emit_err(errors::NestedLifetimes { span: t.span });
                                    }
                                }
                                GenericBound::Outlives(_) => {}
                            }
                        }
                    }
                }
                _ => {}
            }
            self.visit_where_predicate(predicate);
        }
    }

    fn visit_generic_param(&mut self, param: &'a GenericParam) {
        if let GenericParamKind::Lifetime { .. } = param.kind {
            self.check_lifetime(param.ident);
        }
        visit::walk_generic_param(self, param);
    }

    fn visit_param_bound(&mut self, bound: &'a GenericBound, ctxt: BoundKind) {
        if let GenericBound::Trait(poly, modifiers) = bound {
            match (ctxt, modifiers.constness, modifiers.polarity) {
                (BoundKind::SuperTraits, BoundConstness::Never, BoundPolarity::Maybe(_)) => {
                    self.dcx().emit_err(errors::OptionalTraitSupertrait {
                        span: poly.span,
                        path_str: pprust::path_to_string(&poly.trait_ref.path),
                    });
                }
                (BoundKind::TraitObject, BoundConstness::Never, BoundPolarity::Maybe(_)) => {
                    self.dcx().emit_err(errors::OptionalTraitObject { span: poly.span });
                }
                (BoundKind::TraitObject, BoundConstness::Always(_), BoundPolarity::Positive) => {
                    self.dcx().emit_err(errors::ConstBoundTraitObject { span: poly.span });
                }
                (_, BoundConstness::Maybe(span), BoundPolarity::Positive)
                    if let Some(reason) = &self.disallow_tilde_const =>
                {
                    let reason = match reason {
                        DisallowTildeConstContext::Fn(FnKind::Closure(..)) => {
                            errors::TildeConstReason::Closure
                        }
                        DisallowTildeConstContext::Fn(FnKind::Fn(_, ident, ..)) => {
                            errors::TildeConstReason::Function { ident: ident.span }
                        }
                        &DisallowTildeConstContext::Trait(span) => {
                            errors::TildeConstReason::Trait { span }
                        }
                        &DisallowTildeConstContext::TraitImpl(span) => {
                            errors::TildeConstReason::TraitImpl { span }
                        }
                        &DisallowTildeConstContext::Impl(span) => {
                            // FIXME(effects): Consider providing a help message or even a structured
                            // suggestion for moving such bounds to the assoc const fns if available.
                            errors::TildeConstReason::Impl { span }
                        }
                        &DisallowTildeConstContext::TraitAssocTy(span) => {
                            errors::TildeConstReason::TraitAssocTy { span }
                        }
                        &DisallowTildeConstContext::TraitImplAssocTy(span) => {
                            errors::TildeConstReason::TraitImplAssocTy { span }
                        }
                        &DisallowTildeConstContext::InherentAssocTy(span) => {
                            errors::TildeConstReason::InherentAssocTy { span }
                        }
                        DisallowTildeConstContext::TraitObject => {
                            errors::TildeConstReason::TraitObject
                        }
                        DisallowTildeConstContext::Item => errors::TildeConstReason::Item,
                    };
                    self.dcx().emit_err(errors::TildeConstDisallowed { span, reason });
                }
                (
                    _,
                    BoundConstness::Always(_) | BoundConstness::Maybe(_),
                    BoundPolarity::Negative(_) | BoundPolarity::Maybe(_),
                ) => {
                    self.dcx().emit_err(errors::IncompatibleTraitBoundModifiers {
                        span: bound.span(),
                        left: modifiers.constness.as_str(),
                        right: modifiers.polarity.as_str(),
                    });
                }
                _ => {}
            }
        }

        // Negative trait bounds are not allowed to have associated constraints
        if let GenericBound::Trait(trait_ref, modifiers) = bound
            && let BoundPolarity::Negative(_) = modifiers.polarity
            && let Some(segment) = trait_ref.trait_ref.path.segments.last()
        {
            match segment.args.as_deref() {
                Some(ast::GenericArgs::AngleBracketed(args)) => {
                    for arg in &args.args {
                        if let ast::AngleBracketedArg::Constraint(constraint) = arg {
                            self.dcx().emit_err(errors::ConstraintOnNegativeBound {
                                span: constraint.span,
                            });
                        }
                    }
                }
                // The lowered form of parenthesized generic args contains a type binding.
                Some(ast::GenericArgs::Parenthesized(args)) => {
                    self.dcx().emit_err(errors::NegativeBoundWithParentheticalNotation {
                        span: args.span,
                    });
                }
                None => {}
            }
        }

        visit::walk_param_bound(self, bound)
    }

    fn visit_fn(&mut self, fk: FnKind<'a>, span: Span, id: NodeId) {
        // Only associated `fn`s can have `self` parameters.
        let self_semantic = match fk.ctxt() {
            Some(FnCtxt::Assoc(_)) => SelfSemantic::Yes,
            _ => SelfSemantic::No,
        };
        self.check_fn_decl(fk.decl(), self_semantic);

        self.check_c_variadic_type(fk);

        // Functions cannot both be `const async` or `const gen`
        if let Some(&FnHeader {
            constness: Const::Yes(cspan),
            coroutine_kind: Some(coroutine_kind),
            ..
        }) = fk.header()
        {
            let aspan = match coroutine_kind {
                CoroutineKind::Async { span: aspan, .. }
                | CoroutineKind::Gen { span: aspan, .. }
                | CoroutineKind::AsyncGen { span: aspan, .. } => aspan,
            };
            // FIXME(gen_blocks): Report a different error for `const gen`
            self.dcx().emit_err(errors::ConstAndAsync {
                spans: vec![cspan, aspan],
                cspan,
                aspan,
                span,
            });
        }

        if let FnKind::Fn(
            _,
            _,
            FnSig { span: sig_span, header: FnHeader { ext: Extern::Implicit(_), .. }, .. },
            _,
            _,
            _,
        ) = fk
        {
            self.maybe_lint_missing_abi(*sig_span, id);
        }

        // Functions without bodies cannot have patterns.
        if let FnKind::Fn(ctxt, _, sig, _, _, None) = fk {
            Self::check_decl_no_pat(&sig.decl, |span, ident, mut_ident| {
                if mut_ident && matches!(ctxt, FnCtxt::Assoc(_)) {
                    if let Some(ident) = ident {
                        let msg = match ctxt {
                            FnCtxt::Foreign => fluent::ast_passes_pattern_in_foreign,
                            _ => fluent::ast_passes_pattern_in_bodiless,
                        };
                        let diag = BuiltinLintDiag::PatternsInFnsWithoutBody(span, ident);
                        self.lint_buffer.buffer_lint_with_diagnostic(
                            PATTERNS_IN_FNS_WITHOUT_BODY,
                            id,
                            span,
                            msg,
                            diag,
                        )
                    }
                } else {
                    match ctxt {
                        FnCtxt::Foreign => self.dcx().emit_err(errors::PatternInForeign { span }),
                        _ => self.dcx().emit_err(errors::PatternInBodiless { span }),
                    };
                }
            });
        }

        let tilde_const_allowed =
            matches!(fk.header(), Some(FnHeader { constness: ast::Const::Yes(_), .. }))
                || matches!(fk.ctxt(), Some(FnCtxt::Assoc(_)))
                    && self
                        .outer_trait_or_trait_impl
                        .as_ref()
                        .and_then(TraitOrTraitImpl::constness)
                        .is_some();

        let disallowed = (!tilde_const_allowed).then(|| DisallowTildeConstContext::Fn(fk));
        self.with_tilde_const(disallowed, |this| visit::walk_fn(this, fk));
    }

    fn visit_assoc_item(&mut self, item: &'a AssocItem, ctxt: AssocCtxt) {
        if attr::contains_name(&item.attrs, sym::no_mangle) {
            self.check_nomangle_item_asciionly(item.ident, item.span);
        }

        if ctxt == AssocCtxt::Trait || self.outer_trait_or_trait_impl.is_none() {
            self.check_defaultness(item.span, item.kind.defaultness());
        }

        if ctxt == AssocCtxt::Impl {
            match &item.kind {
                AssocItemKind::Const(box ConstItem { expr: None, .. }) => {
                    self.dcx().emit_err(errors::AssocConstWithoutBody {
                        span: item.span,
                        replace_span: self.ending_semi_or_hi(item.span),
                    });
                }
                AssocItemKind::Fn(box Fn { body, .. }) => {
                    if body.is_none() {
                        self.dcx().emit_err(errors::AssocFnWithoutBody {
                            span: item.span,
                            replace_span: self.ending_semi_or_hi(item.span),
                        });
                    }
                }
                AssocItemKind::Type(box TyAlias { bounds, ty, .. }) => {
                    if ty.is_none() {
                        self.dcx().emit_err(errors::AssocTypeWithoutBody {
                            span: item.span,
                            replace_span: self.ending_semi_or_hi(item.span),
                        });
                    }
                    self.check_type_no_bounds(bounds, "`impl`s");
                }
                _ => {}
            }
        }

        if let AssocItemKind::Type(ty_alias) = &item.kind
            && let Err(err) = self.check_type_alias_where_clause_location(ty_alias)
        {
            let sugg = match err.sugg {
                errors::WhereClauseBeforeTypeAliasSugg::Remove { .. } => None,
                errors::WhereClauseBeforeTypeAliasSugg::Move { snippet, right, .. } => {
                    Some((right, snippet))
                }
            };
            self.lint_buffer.buffer_lint_with_diagnostic(
                DEPRECATED_WHERE_CLAUSE_LOCATION,
                item.id,
                err.span,
                fluent::ast_passes_deprecated_where_clause_location,
                BuiltinLintDiag::DeprecatedWhereclauseLocation(sugg),
            );
        }

        if let Some(parent) = &self.outer_trait_or_trait_impl {
            self.visibility_not_permitted(&item.vis, errors::VisibilityNotPermittedNote::TraitImpl);
            if let AssocItemKind::Fn(box Fn { sig, .. }) = &item.kind {
                self.check_trait_fn_not_const(sig.header.constness, parent);
            }
        }

        if let AssocItemKind::Const(..) = item.kind {
            self.check_item_named(item.ident, "const");
        }

        let parent_is_const =
            self.outer_trait_or_trait_impl.as_ref().and_then(TraitOrTraitImpl::constness).is_some();

        match &item.kind {
            AssocItemKind::Fn(box Fn { sig, generics, body, .. })
                if parent_is_const
                    || ctxt == AssocCtxt::Trait
                    || matches!(sig.header.constness, Const::Yes(_)) =>
            {
                self.visit_vis(&item.vis);
                self.visit_ident(item.ident);
                let kind = FnKind::Fn(
                    FnCtxt::Assoc(ctxt),
                    item.ident,
                    sig,
                    &item.vis,
                    generics,
                    body.as_deref(),
                );
                walk_list!(self, visit_attribute, &item.attrs);
                self.visit_fn(kind, item.span, item.id);
            }
            AssocItemKind::Type(_) => {
                let disallowed = (!parent_is_const).then(|| match self.outer_trait_or_trait_impl {
                    Some(TraitOrTraitImpl::Trait { .. }) => {
                        DisallowTildeConstContext::TraitAssocTy(item.span)
                    }
                    Some(TraitOrTraitImpl::TraitImpl { .. }) => {
                        DisallowTildeConstContext::TraitImplAssocTy(item.span)
                    }
                    None => DisallowTildeConstContext::InherentAssocTy(item.span),
                });
                self.with_tilde_const(disallowed, |this| {
                    this.with_in_trait_impl(None, |this| visit::walk_assoc_item(this, item, ctxt))
                })
            }
            _ => self.with_in_trait_impl(None, |this| visit::walk_assoc_item(this, item, ctxt)),
        }
    }
}

/// When encountering an equality constraint in a `where` clause, emit an error. If the code seems
/// like it's setting an associated type, provide an appropriate suggestion.
fn deny_equality_constraints(
    this: &mut AstValidator<'_>,
    predicate: &WhereEqPredicate,
    generics: &Generics,
) {
    let mut err = errors::EqualityInWhere { span: predicate.span, assoc: None, assoc2: None };

    // Given `<A as Foo>::Bar = RhsTy`, suggest `A: Foo<Bar = RhsTy>`.
    if let TyKind::Path(Some(qself), full_path) = &predicate.lhs_ty.kind
        && let TyKind::Path(None, path) = &qself.ty.kind
        && let [PathSegment { ident, args: None, .. }] = &path.segments[..]
    {
        for param in &generics.params {
            if param.ident == *ident
                && let [PathSegment { ident, args, .. }] = &full_path.segments[qself.position..]
            {
                // Make a new `Path` from `foo::Bar` to `Foo<Bar = RhsTy>`.
                let mut assoc_path = full_path.clone();
                // Remove `Bar` from `Foo::Bar`.
                assoc_path.segments.pop();
                let len = assoc_path.segments.len() - 1;
                let gen_args = args.as_deref().cloned();
                // Build `<Bar = RhsTy>`.
                let arg = AngleBracketedArg::Constraint(AssocConstraint {
                    id: rustc_ast::node_id::DUMMY_NODE_ID,
                    ident: *ident,
                    gen_args,
                    kind: AssocConstraintKind::Equality { term: predicate.rhs_ty.clone().into() },
                    span: ident.span,
                });
                // Add `<Bar = RhsTy>` to `Foo`.
                match &mut assoc_path.segments[len].args {
                    Some(args) => match args.deref_mut() {
                        GenericArgs::Parenthesized(_) => continue,
                        GenericArgs::AngleBracketed(args) => {
                            args.args.push(arg);
                        }
                    },
                    empty_args => {
                        *empty_args = Some(
                            AngleBracketedArgs { span: ident.span, args: thin_vec![arg] }.into(),
                        );
                    }
                }
                err.assoc = Some(errors::AssociatedSuggestion {
                    span: predicate.span,
                    ident: *ident,
                    param: param.ident,
                    path: pprust::path_to_string(&assoc_path),
                })
            }
        }
    }

    let mut suggest =
        |poly: &PolyTraitRef, potential_assoc: &PathSegment, predicate: &WhereEqPredicate| {
            if let [trait_segment] = &poly.trait_ref.path.segments[..] {
                let assoc = pprust::path_to_string(&ast::Path::from_ident(potential_assoc.ident));
                let ty = pprust::ty_to_string(&predicate.rhs_ty);
                let (args, span) = match &trait_segment.args {
                    Some(args) => match args.deref() {
                        ast::GenericArgs::AngleBracketed(args) => {
                            let Some(arg) = args.args.last() else {
                                return;
                            };
                            (format!(", {assoc} = {ty}"), arg.span().shrink_to_hi())
                        }
                        _ => return,
                    },
                    None => (format!("<{assoc} = {ty}>"), trait_segment.span().shrink_to_hi()),
                };
                let removal_span = if generics.where_clause.predicates.len() == 1 {
                    // We're removing th eonly where bound left, remove the whole thing.
                    generics.where_clause.span
                } else {
                    let mut span = predicate.span;
                    let mut prev: Option<Span> = None;
                    let mut preds = generics.where_clause.predicates.iter().peekable();
                    // Find the predicate that shouldn't have been in the where bound list.
                    while let Some(pred) = preds.next() {
                        if let WherePredicate::EqPredicate(pred) = pred
                            && pred.span == predicate.span
                        {
                            if let Some(next) = preds.peek() {
                                // This is the first predicate, remove the trailing comma as well.
                                span = span.with_hi(next.span().lo());
                            } else if let Some(prev) = prev {
                                // Remove the previous comma as well.
                                span = span.with_lo(prev.hi());
                            }
                        }
                        prev = Some(pred.span());
                    }
                    span
                };
                err.assoc2 = Some(errors::AssociatedSuggestion2 {
                    span,
                    args,
                    predicate: removal_span,
                    trait_segment: trait_segment.ident,
                    potential_assoc: potential_assoc.ident,
                });
            }
        };

    if let TyKind::Path(None, full_path) = &predicate.lhs_ty.kind {
        // Given `A: Foo, Foo::Bar = RhsTy`, suggest `A: Foo<Bar = RhsTy>`.
        for bounds in generics.params.iter().map(|p| &p.bounds).chain(
            generics.where_clause.predicates.iter().filter_map(|pred| match pred {
                WherePredicate::BoundPredicate(p) => Some(&p.bounds),
                _ => None,
            }),
        ) {
            for bound in bounds {
                if let GenericBound::Trait(poly, TraitBoundModifiers::NONE) = bound {
                    if full_path.segments[..full_path.segments.len() - 1]
                        .iter()
                        .map(|segment| segment.ident.name)
                        .zip(poly.trait_ref.path.segments.iter().map(|segment| segment.ident.name))
                        .all(|(a, b)| a == b)
                        && let Some(potential_assoc) = full_path.segments.iter().last()
                    {
                        suggest(poly, potential_assoc, predicate);
                    }
                }
            }
        }
        // Given `A: Foo, A::Bar = RhsTy`, suggest `A: Foo<Bar = RhsTy>`.
        if let [potential_param, potential_assoc] = &full_path.segments[..] {
            for (ident, bounds) in generics.params.iter().map(|p| (p.ident, &p.bounds)).chain(
                generics.where_clause.predicates.iter().filter_map(|pred| match pred {
                    WherePredicate::BoundPredicate(p)
                        if let ast::TyKind::Path(None, path) = &p.bounded_ty.kind
                            && let [segment] = &path.segments[..] =>
                    {
                        Some((segment.ident, &p.bounds))
                    }
                    _ => None,
                }),
            ) {
                if ident == potential_param.ident {
                    for bound in bounds {
                        if let ast::GenericBound::Trait(poly, TraitBoundModifiers::NONE) = bound {
                            suggest(poly, potential_assoc, predicate);
                        }
                    }
                }
            }
        }
    }
    this.dcx().emit_err(err);
}

pub fn check_crate(
    session: &Session,
    features: &Features,
    krate: &Crate,
    lints: &mut LintBuffer,
) -> bool {
    let mut validator = AstValidator {
        session,
        features,
        extern_mod: None,
        outer_trait_or_trait_impl: None,
        has_proc_macro_decls: false,
        outer_impl_trait: None,
        disallow_tilde_const: Some(DisallowTildeConstContext::Item),
        is_impl_trait_banned: false,
        lint_buffer: lints,
    };
    visit::walk_crate(&mut validator, krate);

    validator.has_proc_macro_decls
}
