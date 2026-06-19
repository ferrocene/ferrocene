use rustc_hir::def::DefKind;
use rustc_hir::def_id::DefId;
use rustc_hir::{
    ForeignItem, ForeignItemKind, Item, ItemKind, Node, TraitFn, TraitItem, TraitItemKind,
};
use rustc_macros::{HashStable, TyDecodable, TyEncodable};
use rustc_middle::ty::{self, TyCtxt};
use rustc_span::{Span, Symbol, sym};
use tracing::{info, instrument};

#[derive(Clone, TyEncodable, TyDecodable, HashStable, Debug)]
pub struct Validated {
    // prevalidated: bool,
    // FIXME: exported_constraint, validated, standards, qualification levels, entrypoint
}

const VALIDATED_ATTR: &[Symbol; 2] = &[sym::ferrocene, sym::prevalidated];

#[derive(Debug)]
pub enum ValidatedStatus {
    Validated {
        /// `annotation` is None if this is the `main` entrypoint, or if it was synthesized by
        /// the compiler.
        annotation: Option<Span>,
        /// We want to distinguish between items that are *directly* annotated and items that are
        /// *indirectly* annotated.
        /// Indirecty annotated items can sometimes have no meaning: macros, `use` statements, etc.
        /// But we may wish to give them a meaning in the future if they're directly annotated.
        /// So: only consider indirect annotations for certain kinds of items.
        inherited: bool,
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
#[instrument(skip(tcx), ret)]
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
        return ValidatedStatus::Validated { annotation: None, inherited: false };
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
                    return ValidatedStatus::Validated { annotation: None, inherited: false, };
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

    if let Some(annotation) = any_parent_is_validated(tcx, owner) {
        return annotation;
    }

    // HACK: `feature(delegation)` is horribly buggy and causes infinite cycles within the query
    // system itself. While we wait for upstream to fix it, work around the issue.
    // See https://github.com/rust-lang/rust/pull/154387 for more information.
    let upstream_has_bugs = !tcx.hir_crate(()).delayed_ids.is_empty();
    let is_main = upstream_has_bugs || tcx.entry_fn(()).map_or(false, |(main, _)| main == owner);
    // We normally assume main functions are validated, but not for the
    // shim generated by `rustc --test`.
    let main_is_validated = is_main && !tcx.sess.opts.test;

    if upstream_has_bugs {
        // Avoid trying to run our lint on items with delegation errors.
        assert!(tcx.dcx().has_errors().is_some() || tcx.features().fn_delegation());
        ValidatedStatus::WorkaroundDelegationBugs
    } else if main_is_validated {
        ValidatedStatus::Validated { annotation: None, inherited: false }
    } else {
        ValidatedStatus::Unvalidated
    }
}

/// Check if this item or any of its parents are validated.
fn any_parent_is_validated(tcx: TyCtxt<'_>, item: DefId) -> Option<ValidatedStatus> {
    let mut current = item;
    loop {
        // Check if it's possible for this item to have attributes.
        // If not, skip it so `get_attrs` doesn't panic.
        match tcx.def_kind(current) {
            DefKind::ForeignMod => {
                current = tcx.parent(current);
                continue;
            }
            _ => {}
        }
        // Check if the current item has an annotation
        if let Some(attr) = tcx.get_attrs_by_path(current, VALIDATED_ATTR).next() {
            return Some(ValidatedStatus::Validated {
                annotation: Some(attr.span()),
                inherited: current != item,
            });
        }

        if current.is_crate_root() {
            return None;
        }

        current = tcx.parent(current);
    }
}
