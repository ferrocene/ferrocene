use rustc_middle::mir::mono::MonoItem;
use rustc_middle::ty::TyCtxt;

use crate::collector::{MonoItemCollectionStrategy, collect_roots};

/// Find all validated mono roots. See the [module docs](crate::collector#discovering-roots).
pub fn collect_validated_roots<'tcx>(tcx: TyCtxt<'tcx>) -> Vec<MonoItem<'tcx>> {
    let roots = collect_roots(tcx, MonoItemCollectionStrategy::Validated);
    // roots.sort(); // for query stability
    roots
}
