use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_hir::{
    ForeignItem, ForeignItemKind, Item, ItemKind, Node, TraitFn, TraitItem, TraitItemKind,
};
use rustc_macros::{HashStable, TyDecodable, TyEncodable};
use rustc_middle::ty::{self, TyCtxt};
use rustc_span::{Span, Symbol, sym};
use tracing::info;

#[derive(Clone, TyEncodable, TyDecodable, HashStable, Debug)]
pub struct Validated {
    // prevalidated: bool,
    // FIXME: exported_constraint, validated, standards, qualification levels, entrypoint
}

const VALIDATED_ATTR: &[Symbol; 2] = &[sym::ferrocene, sym::prevalidated];

pub enum ValidatedStatus {
    /// `annotation` is None IFF this is the `main` entrypoint.
    Validated {
        annotation: Option<Span>,
    },
    Unvalidated,
    WorkaroundDelegationBugs,
}

impl ValidatedStatus {
    pub fn validated(self) -> bool {
        match self {
            ValidatedStatus::Validated { .. } => true,
            ValidatedStatus::WorkaroundDelegationBugs | ValidatedStatus::Unvalidated => false,
        }
    }
}

/// Shared between `rustc_lint` and `rustc_codegen_ssa` attr parsing.
///
/// This analysis needs to be conservative. If you don't have enough information to determine the
/// status, assume it's unvalidated.
pub fn item_is_validated(tcx: TyCtxt<'_>, def_id: DefId) -> ValidatedStatus {
    // A closure is validated if the function it's defined in is validated.
    let owner = tcx.typeck_root_def_id(def_id);

    // Skip items synthesized by the compiler.
    let synthetic = match tcx.def_kind(owner) {
        DefKind::Ctor(..) | DefKind::SyntheticCoroutineBody => true,
        // NOTE: intrinsics might have a "fallback body" that is used as a default if the codegen
        // backend doesn't override it.
        _ => tcx.intrinsic(owner).is_some_and(|def| def.must_be_overridden),
    };
    if synthetic {
        info!("skipping synthetic item {owner:?}");
        return ValidatedStatus::Validated { annotation: None };
    }

    // Skip intrinsics, extern functions, and associated functions with no default.
    // We only have to do this for local DefIds; nothing makes it to post-mono without a body unless
    // it's an intrinsic.
    if let Some(local) = owner.as_local() {
        match tcx.hir_node_by_def_id(local) {
            Node::Item(Item { kind: ItemKind::Fn { has_body: false, .. }, .. })
                // FIXME: ForeignItems should be an exported_constraint
                | Node::ForeignItem(ForeignItem { kind: ForeignItemKind::Fn(..), .. })
                | Node::TraitItem(TraitItem { kind: TraitItemKind::Fn(_, TraitFn::Required(_)), .. }) => {
                    info!("skipping item {owner:?}");
                    return ValidatedStatus::Validated { annotation: None };
                }
            _ => {},
        }

        // Check if this is an associated function from a `derive`.
        let parent = tcx.parent(owner);
        tracing::debug!("parent={parent:?}, kind={:?}", tcx.def_kind(parent));
        #[allow(deprecated)]
        if matches!(tcx.def_kind(parent), DefKind::Impl { .. }) {
            tracing::debug!("attrs={:?}", tcx.get_all_attrs(parent));
        }

        let derived = matches!(tcx.def_kind(parent), DefKind::Impl { .. })
            && tcx.is_automatically_derived(parent);
        if derived {
            // Builtin macros are covered under our compiler qualification and considered verified.
            // We currently don't support proc-macros; assume them to be unverified.
            if !tcx.is_builtin_derived(parent) {
                return ValidatedStatus::Unvalidated;
            }

            let self_ty = tcx.type_of(parent);
            info!("checking {self_ty:?}");
            // FIXME: need to try to normalize this type so we handle aliases and associated types
            let unwrapped_ty = self_ty.skip_binder().peel_refs();
            match unwrapped_ty.kind() {
                ty::Adt(adt_def, _) => {
                    let self_id = adt_def.did();
                    info!(
                        "{} is marked #[automatically_derived], checking {}",
                        tcx.def_path_str(def_id),
                        tcx.def_path_str(self_id)
                    );
                    return item_is_validated(tcx, self_id);
                }
                ty::Error(..) => {}
                _ if unwrapped_ty.is_primitive_ty() => {}
                // We don't know how to resolve this back to a type.
                // For now, just assume it's unvalidated.
                _ => {
                    return ValidatedStatus::Unvalidated;
                }
            }
        }
    }

    // HACK: `feature(delegation)` is horribly buggy and causes infinite cycles within the query
    // system itself. While we wait for upstream to fix it, work around the issue.
    // See https://github.com/rust-lang/rust/pull/154387 for more information.
    let upstream_has_bugs = !tcx.hir_crate(()).delayed_ids.is_empty();
    let is_main = upstream_has_bugs || tcx.entry_fn(()).map_or(false, |(main, _)| main == owner);
    // We normally assume main functions are validated, but not for the
    // shim generated by `rustc --test`.
    let main_is_validated = is_main && !tcx.sess.opts.test;

    let annotation = tcx.get_attrs_by_path(owner, VALIDATED_ATTR).next();
    if annotation.is_some() {
        ValidatedStatus::Validated { annotation: annotation.map(|attr| attr.span()) }
    } else if upstream_has_bugs {
        // Avoid trying to run our lint on items with delegation errors.
        assert!(tcx.dcx().has_errors().is_some() || tcx.features().fn_delegation());
        ValidatedStatus::WorkaroundDelegationBugs
    } else if main_is_validated {
        ValidatedStatus::Validated { annotation: None }
    } else {
        ValidatedStatus::Unvalidated
    }
}
