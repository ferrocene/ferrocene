use crate::clean::auto_trait::AutoTraitFinder;
use crate::clean::blanket_impl::BlanketImplFinder;
use crate::clean::render_macro_matchers::render_macro_matcher;
use crate::clean::{
    clean_doc_module, clean_middle_const, clean_middle_region, clean_middle_ty, inline, Crate,
    ExternalCrate, Generic, GenericArg, GenericArgs, ImportSource, Item, ItemKind, Lifetime, Path,
    PathSegment, Primitive, PrimitiveType, Term, Type, TypeBinding, TypeBindingKind,
};
use crate::core::DocContext;
use crate::html::format::visibility_to_src_with_space;

use rustc_ast as ast;
use rustc_ast::tokenstream::TokenTree;
use rustc_hir as hir;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::def_id::{DefId, LocalDefId, LOCAL_CRATE};
use rustc_metadata::rendered_const;
use rustc_middle::mir;
use rustc_middle::ty::{self, GenericArgKind, GenericArgsRef, TyCtxt};
use rustc_span::symbol::{kw, sym, Symbol};
use std::fmt::Write as _;
use std::mem;
use std::sync::LazyLock as Lazy;
use thin_vec::{thin_vec, ThinVec};

#[cfg(test)]
mod tests;

pub(crate) fn krate(cx: &mut DocContext<'_>) -> Crate {
    let module = crate::visit_ast::RustdocVisitor::new(cx).visit();

    // Clean the crate, translating the entire librustc_ast AST to one that is
    // understood by rustdoc.
    let mut module = clean_doc_module(&module, cx);

    match *module.kind {
        ItemKind::ModuleItem(ref module) => {
            for it in &module.items {
                // `compiler_builtins` should be masked too, but we can't apply
                // `#[doc(masked)]` to the injected `extern crate` because it's unstable.
                if cx.tcx.is_compiler_builtins(it.item_id.krate()) {
                    cx.cache.masked_crates.insert(it.item_id.krate());
                } else if it.is_extern_crate()
                    && it.attrs.has_doc_flag(sym::masked)
                    && let Some(def_id) = it.item_id.as_def_id()
                    && let Some(local_def_id) = def_id.as_local()
                    && let Some(cnum) = cx.tcx.extern_mod_stmt_cnum(local_def_id)
                {
                    cx.cache.masked_crates.insert(cnum);
                }
            }
        }
        _ => unreachable!(),
    }

    let local_crate = ExternalCrate { crate_num: LOCAL_CRATE };
    let primitives = local_crate.primitives(cx.tcx);
    let keywords = local_crate.keywords(cx.tcx);
    {
        let ItemKind::ModuleItem(ref mut m) = *module.kind else { unreachable!() };
        m.items.extend(primitives.iter().map(|&(def_id, prim)| {
            Item::from_def_id_and_parts(
                def_id,
                Some(prim.as_sym()),
                ItemKind::PrimitiveItem(prim),
                cx,
            )
        }));
        m.items.extend(keywords.into_iter().map(|(def_id, kw)| {
            Item::from_def_id_and_parts(def_id, Some(kw), ItemKind::KeywordItem, cx)
        }));
    }

    Crate { module, external_traits: cx.external_traits.clone() }
}

pub(crate) fn ty_args_to_args<'tcx>(
    cx: &mut DocContext<'tcx>,
    args: ty::Binder<'tcx, &'tcx [ty::GenericArg<'tcx>]>,
    has_self: bool,
    container: Option<DefId>,
) -> Vec<GenericArg> {
    let mut skip_first = has_self;
    let mut ret_val =
        Vec::with_capacity(args.skip_binder().len().saturating_sub(if skip_first { 1 } else { 0 }));

    ret_val.extend(args.iter().enumerate().filter_map(|(index, kind)| {
        match kind.skip_binder().unpack() {
            GenericArgKind::Lifetime(lt) => {
                Some(GenericArg::Lifetime(clean_middle_region(lt).unwrap_or(Lifetime::elided())))
            }
            GenericArgKind::Type(_) if skip_first => {
                skip_first = false;
                None
            }
            GenericArgKind::Type(ty) => Some(GenericArg::Type(clean_middle_ty(
                kind.rebind(ty),
                cx,
                None,
                container.map(|container| crate::clean::ContainerTy::Regular {
                    ty: container,
                    args,
                    has_self,
                    arg: index,
                }),
            ))),
            GenericArgKind::Const(ct) => {
                Some(GenericArg::Const(Box::new(clean_middle_const(kind.rebind(ct), cx))))
            }
        }
    }));
    ret_val
}

fn external_generic_args<'tcx>(
    cx: &mut DocContext<'tcx>,
    did: DefId,
    has_self: bool,
    bindings: ThinVec<TypeBinding>,
    ty_args: ty::Binder<'tcx, GenericArgsRef<'tcx>>,
) -> GenericArgs {
    let args = ty_args_to_args(cx, ty_args.map_bound(|args| &args[..]), has_self, Some(did));

    if cx.tcx.fn_trait_kind_from_def_id(did).is_some() {
        let ty = ty_args
            .iter()
            .nth(if has_self { 1 } else { 0 })
            .unwrap()
            .map_bound(|arg| arg.expect_ty());
        let inputs =
            // The trait's first substitution is the one after self, if there is one.
            match ty.skip_binder().kind() {
                ty::Tuple(tys) => tys.iter().map(|t| clean_middle_ty(ty.rebind(t), cx, None, None)).collect::<Vec<_>>().into(),
                _ => return GenericArgs::AngleBracketed { args: args.into(), bindings },
            };
        let output = bindings.into_iter().next().and_then(|binding| match binding.kind {
            TypeBindingKind::Equality { term: Term::Type(ty) } if ty != Type::Tuple(Vec::new()) => {
                Some(Box::new(ty))
            }
            _ => None,
        });
        GenericArgs::Parenthesized { inputs, output }
    } else {
        GenericArgs::AngleBracketed { args: args.into(), bindings }
    }
}

pub(super) fn external_path<'tcx>(
    cx: &mut DocContext<'tcx>,
    did: DefId,
    has_self: bool,
    bindings: ThinVec<TypeBinding>,
    args: ty::Binder<'tcx, GenericArgsRef<'tcx>>,
) -> Path {
    let def_kind = cx.tcx.def_kind(did);
    let name = cx.tcx.opt_item_name(did).unwrap_or(kw::Empty);
    Path {
        res: Res::Def(def_kind, did),
        segments: thin_vec![PathSegment {
            name,
            args: external_generic_args(cx, did, has_self, bindings, args),
        }],
    }
}

/// Remove the generic arguments from a path.
pub(crate) fn strip_path_generics(mut path: Path) -> Path {
    for ps in path.segments.iter_mut() {
        ps.args = GenericArgs::AngleBracketed { args: Default::default(), bindings: ThinVec::new() }
    }

    path
}

pub(crate) fn qpath_to_string(p: &hir::QPath<'_>) -> String {
    let segments = match *p {
        hir::QPath::Resolved(_, path) => &path.segments,
        hir::QPath::TypeRelative(_, segment) => return segment.ident.to_string(),
        hir::QPath::LangItem(lang_item, ..) => return lang_item.name().to_string(),
    };

    let mut s = String::new();
    for (i, seg) in segments.iter().enumerate() {
        if i > 0 {
            s.push_str("::");
        }
        if seg.ident.name != kw::PathRoot {
            s.push_str(seg.ident.as_str());
        }
    }
    s
}

pub(crate) fn build_deref_target_impls(
    cx: &mut DocContext<'_>,
    items: &[Item],
    ret: &mut Vec<Item>,
) {
    let tcx = cx.tcx;

    for item in items {
        let target = match *item.kind {
            ItemKind::AssocTypeItem(ref t, _) => &t.type_,
            _ => continue,
        };

        if let Some(prim) = target.primitive_type() {
            let _prof_timer = tcx.sess.prof.generic_activity("build_primitive_inherent_impls");
            for did in prim.impls(tcx).filter(|did| !did.is_local()) {
                inline::build_impl(cx, did, None, ret);
            }
        } else if let Type::Path { path } = target {
            let did = path.def_id();
            if !did.is_local() {
                inline::build_impls(cx, did, None, ret);
            }
        }
    }
}

pub(crate) fn name_from_pat(p: &hir::Pat<'_>) -> Symbol {
    use rustc_hir::*;
    debug!("trying to get a name from pattern: {p:?}");

    Symbol::intern(&match p.kind {
        PatKind::Wild | PatKind::Struct(..) => return kw::Underscore,
        PatKind::Binding(_, _, ident, _) => return ident.name,
        PatKind::TupleStruct(ref p, ..) | PatKind::Path(ref p) => qpath_to_string(p),
        PatKind::Or(pats) => {
            pats.iter().map(|p| name_from_pat(p).to_string()).collect::<Vec<String>>().join(" | ")
        }
        PatKind::Tuple(elts, _) => format!(
            "({})",
            elts.iter().map(|p| name_from_pat(p).to_string()).collect::<Vec<String>>().join(", ")
        ),
        PatKind::Box(p) => return name_from_pat(&*p),
        PatKind::Ref(p, _) => return name_from_pat(&*p),
        PatKind::Lit(..) => {
            warn!(
                "tried to get argument name from PatKind::Lit, which is silly in function arguments"
            );
            return Symbol::intern("()");
        }
        PatKind::Range(..) => return kw::Underscore,
        PatKind::Slice(begin, ref mid, end) => {
            let begin = begin.iter().map(|p| name_from_pat(p).to_string());
            let mid = mid.as_ref().map(|p| format!("..{}", name_from_pat(&**p))).into_iter();
            let end = end.iter().map(|p| name_from_pat(p).to_string());
            format!("[{}]", begin.chain(mid).chain(end).collect::<Vec<_>>().join(", "))
        }
    })
}

pub(crate) fn print_const(cx: &DocContext<'_>, n: ty::Const<'_>) -> String {
    match n.kind() {
        ty::ConstKind::Unevaluated(ty::UnevaluatedConst { def, args: _ }) => {
            let s = if let Some(def) = def.as_local() {
                rendered_const(cx.tcx, cx.tcx.hir().body_owned_by(def))
            } else {
                inline::print_inlined_const(cx.tcx, def)
            };

            s
        }
        // array lengths are obviously usize
        ty::ConstKind::Value(ty::ValTree::Leaf(scalar))
            if *n.ty().kind() == ty::Uint(ty::UintTy::Usize) =>
        {
            scalar.to_string()
        }
        _ => n.to_string(),
    }
}

pub(crate) fn print_evaluated_const(
    tcx: TyCtxt<'_>,
    def_id: DefId,
    with_underscores: bool,
    with_type: bool,
) -> Option<String> {
    tcx.const_eval_poly(def_id).ok().and_then(|val| {
        let ty = tcx.type_of(def_id).instantiate_identity();
        match (val, ty.kind()) {
            (_, &ty::Ref(..)) => None,
            (mir::ConstValue::Scalar(_), &ty::Adt(_, _)) => None,
            (mir::ConstValue::Scalar(_), _) => {
                let const_ = mir::Const::from_value(val, ty);
                Some(print_const_with_custom_print_scalar(tcx, const_, with_underscores, with_type))
            }
            _ => None,
        }
    })
}

fn format_integer_with_underscore_sep(num: &str) -> String {
    let num_chars: Vec<_> = num.chars().collect();
    let mut num_start_index = if num_chars.get(0) == Some(&'-') { 1 } else { 0 };
    let chunk_size = match num[num_start_index..].as_bytes() {
        [b'0', b'b' | b'x', ..] => {
            num_start_index += 2;
            4
        }
        [b'0', b'o', ..] => {
            num_start_index += 2;
            let remaining_chars = num_chars.len() - num_start_index;
            if remaining_chars <= 6 {
                // don't add underscores to Unix permissions like 0755 or 100755
                return num.to_string();
            }
            3
        }
        _ => 3,
    };

    num_chars[..num_start_index]
        .iter()
        .chain(num_chars[num_start_index..].rchunks(chunk_size).rev().intersperse(&['_']).flatten())
        .collect()
}

fn print_const_with_custom_print_scalar<'tcx>(
    tcx: TyCtxt<'tcx>,
    ct: mir::Const<'tcx>,
    with_underscores: bool,
    with_type: bool,
) -> String {
    // Use a slightly different format for integer types which always shows the actual value.
    // For all other types, fallback to the original `pretty_print_const`.
    match (ct, ct.ty().kind()) {
        (mir::Const::Val(mir::ConstValue::Scalar(int), _), ty::Uint(ui)) => {
            let mut output = if with_underscores {
                format_integer_with_underscore_sep(&int.to_string())
            } else {
                int.to_string()
            };
            if with_type {
                output += ui.name_str();
            }
            output
        }
        (mir::Const::Val(mir::ConstValue::Scalar(int), _), ty::Int(i)) => {
            let ty = ct.ty();
            let size = tcx.layout_of(ty::ParamEnv::empty().and(ty)).unwrap().size;
            let data = int.assert_bits(size);
            let sign_extended_data = size.sign_extend(data) as i128;
            let mut output = if with_underscores {
                format_integer_with_underscore_sep(&sign_extended_data.to_string())
            } else {
                sign_extended_data.to_string()
            };
            if with_type {
                output += i.name_str();
            }
            output
        }
        _ => ct.to_string(),
    }
}

pub(crate) fn is_literal_expr(tcx: TyCtxt<'_>, hir_id: hir::HirId) -> bool {
    if let hir::Node::Expr(expr) = tcx.hir().get(hir_id) {
        if let hir::ExprKind::Lit(_) = &expr.kind {
            return true;
        }

        if let hir::ExprKind::Unary(hir::UnOp::Neg, expr) = &expr.kind &&
            let hir::ExprKind::Lit(_) = &expr.kind
        {
            return true;
        }
    }

    false
}

/// Given a type Path, resolve it to a Type using the TyCtxt
pub(crate) fn resolve_type(cx: &mut DocContext<'_>, path: Path) -> Type {
    debug!("resolve_type({path:?})");

    match path.res {
        Res::PrimTy(p) => Primitive(PrimitiveType::from(p)),
        Res::SelfTyParam { .. } | Res::SelfTyAlias { .. } if path.segments.len() == 1 => {
            Generic(kw::SelfUpper)
        }
        Res::Def(DefKind::TyParam, _) if path.segments.len() == 1 => Generic(path.segments[0].name),
        _ => {
            let _ = register_res(cx, path.res);
            Type::Path { path }
        }
    }
}

pub(crate) fn get_auto_trait_and_blanket_impls(
    cx: &mut DocContext<'_>,
    item_def_id: DefId,
) -> impl Iterator<Item = Item> {
    let auto_impls = cx
        .sess()
        .prof
        .generic_activity("get_auto_trait_impls")
        .run(|| AutoTraitFinder::new(cx).get_auto_trait_impls(item_def_id));
    let blanket_impls = cx
        .sess()
        .prof
        .generic_activity("get_blanket_impls")
        .run(|| BlanketImplFinder { cx }.get_blanket_impls(item_def_id));
    auto_impls.into_iter().chain(blanket_impls)
}

/// If `res` has a documentation page associated, store it in the cache.
///
/// This is later used by [`href()`] to determine the HTML link for the item.
///
/// [`href()`]: crate::html::format::href
pub(crate) fn register_res(cx: &mut DocContext<'_>, res: Res) -> DefId {
    use DefKind::*;
    debug!("register_res({res:?})");

    let (kind, did) = match res {
        Res::Def(
            kind @ (AssocTy
            | AssocFn
            | AssocConst
            | Variant
            | Fn
            | TyAlias { .. }
            | Enum
            | Trait
            | Struct
            | Union
            | Mod
            | ForeignTy
            | Const
            | Static(_)
            | Macro(..)
            | TraitAlias),
            did,
        ) => (kind.into(), did),

        _ => panic!("register_res: unexpected {res:?}"),
    };
    if did.is_local() {
        return did;
    }
    inline::record_extern_fqn(cx, did, kind);
    did
}

pub(crate) fn resolve_use_source(cx: &mut DocContext<'_>, path: Path) -> ImportSource {
    ImportSource {
        did: if path.res.opt_def_id().is_none() { None } else { Some(register_res(cx, path.res)) },
        path,
    }
}

pub(crate) fn enter_impl_trait<'tcx, F, R>(cx: &mut DocContext<'tcx>, f: F) -> R
where
    F: FnOnce(&mut DocContext<'tcx>) -> R,
{
    let old_bounds = mem::take(&mut cx.impl_trait_bounds);
    let r = f(cx);
    assert!(cx.impl_trait_bounds.is_empty());
    cx.impl_trait_bounds = old_bounds;
    r
}

/// Find the nearest parent module of a [`DefId`].
pub(crate) fn find_nearest_parent_module(tcx: TyCtxt<'_>, def_id: DefId) -> Option<DefId> {
    if def_id.is_top_level_module() {
        // The crate root has no parent. Use it as the root instead.
        Some(def_id)
    } else {
        let mut current = def_id;
        // The immediate parent might not always be a module.
        // Find the first parent which is.
        while let Some(parent) = tcx.opt_parent(current) {
            if tcx.def_kind(parent) == DefKind::Mod {
                return Some(parent);
            }
            current = parent;
        }
        None
    }
}

/// Checks for the existence of `hidden` in the attribute below if `flag` is `sym::hidden`:
///
/// ```
/// #[doc(hidden)]
/// pub fn foo() {}
/// ```
///
/// This function exists because it runs on `hir::Attributes` whereas the other is a
/// `clean::Attributes` method.
pub(crate) fn has_doc_flag(tcx: TyCtxt<'_>, did: DefId, flag: Symbol) -> bool {
    tcx.get_attrs(did, sym::doc).any(|attr| {
        attr.meta_item_list().map_or(false, |l| rustc_attr::list_contains_name(&l, flag))
    })
}

/// A link to `doc.rust-lang.org` that includes the channel name. Use this instead of manual links
/// so that the channel is consistent.
///
/// Set by `bootstrap::Builder::doc_rust_lang_org_channel` in order to keep tests passing on beta/stable.
pub(crate) const DOC_RUST_LANG_ORG_CHANNEL: &str = env!("DOC_RUST_LANG_ORG_CHANNEL");
pub(crate) static DOC_CHANNEL: Lazy<&'static str> =
    Lazy::new(|| DOC_RUST_LANG_ORG_CHANNEL.rsplit("/").filter(|c| !c.is_empty()).next().unwrap());

/// Render a sequence of macro arms in a format suitable for displaying to the user
/// as part of an item declaration.
pub(super) fn render_macro_arms<'a>(
    tcx: TyCtxt<'_>,
    matchers: impl Iterator<Item = &'a TokenTree>,
    arm_delim: &str,
) -> String {
    let mut out = String::new();
    for matcher in matchers {
        writeln!(
            out,
            "    {matcher} => {{ ... }}{arm_delim}",
            matcher = render_macro_matcher(tcx, matcher),
        )
        .unwrap();
    }
    out
}

pub(super) fn display_macro_source(
    cx: &mut DocContext<'_>,
    name: Symbol,
    def: &ast::MacroDef,
    def_id: DefId,
    vis: ty::Visibility<DefId>,
) -> String {
    // Extract the spans of all matchers. They represent the "interface" of the macro.
    let matchers = def.body.tokens.chunks(4).map(|arm| &arm[0]);

    if def.macro_rules {
        format!("macro_rules! {name} {{\n{arms}}}", arms = render_macro_arms(cx.tcx, matchers, ";"))
    } else {
        if matchers.len() <= 1 {
            format!(
                "{vis}macro {name}{matchers} {{\n    ...\n}}",
                vis = visibility_to_src_with_space(Some(vis), cx.tcx, def_id),
                matchers = matchers
                    .map(|matcher| render_macro_matcher(cx.tcx, matcher))
                    .collect::<String>(),
            )
        } else {
            format!(
                "{vis}macro {name} {{\n{arms}}}",
                vis = visibility_to_src_with_space(Some(vis), cx.tcx, def_id),
                arms = render_macro_arms(cx.tcx, matchers, ","),
            )
        }
    }
}

pub(crate) fn inherits_doc_hidden(
    tcx: TyCtxt<'_>,
    mut def_id: LocalDefId,
    stop_at: Option<LocalDefId>,
) -> bool {
    let hir = tcx.hir();
    while let Some(id) = tcx.opt_local_parent(def_id) {
        if let Some(stop_at) = stop_at && id == stop_at {
            return false;
        }
        def_id = id;
        if tcx.is_doc_hidden(def_id.to_def_id()) {
            return true;
        } else if let Some(node) = hir.find_by_def_id(def_id) &&
            matches!(
                node,
                hir::Node::Item(hir::Item { kind: hir::ItemKind::Impl(_), .. }),
            )
        {
            // `impl` blocks stand a bit on their own: unless they have `#[doc(hidden)]` directly
            // on them, they don't inherit it from the parent context.
            return false;
        }
    }
    false
}

#[inline]
pub(crate) fn should_ignore_res(res: Res) -> bool {
    matches!(res, Res::Def(DefKind::Ctor(..), _) | Res::SelfCtor(..))
}
