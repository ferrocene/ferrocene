use rustc_data_structures::unord::UnordSet;
use rustc_middle::mir::mono::MonoItem;
use rustc_middle::ty::TyCtxt;

use crate::collector::{MonoItemCollectionStrategy, collect_roots};

/// Find all validated mono roots. See the [module docs](crate::collector#discovering-roots).
/// NOTE: returns an unordered set for query stability.
pub fn collect_validated_roots<'tcx>(tcx: TyCtxt<'tcx>) -> UnordSet<MonoItem<'tcx>> {
    let roots = collect_roots(tcx, MonoItemCollectionStrategy::Validated);
    roots.into_iter().collect()
}
