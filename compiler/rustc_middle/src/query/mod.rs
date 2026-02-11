use rustc_data_structures::jobserver::Proxy;
use rustc_hir::def_id::LocalDefId;
use rustc_query_system::query::QuerySideEffect;

pub use self::caches::{
    DefIdCache, DefaultCache, QueryCache, QueryCacheKey, SingleCache, VecCache,
};
pub use self::job::{QueryInfo, QueryJob, QueryJobId, QueryLatch, QueryWaiter};
pub use self::keys::{AsLocalKey, Key, LocalCrate};
pub use self::plumbing::{
    ActiveKeyStatus, CycleError, CycleErrorHandling, IntoQueryParam, QueryMode, QueryState,
    TyCtxtAt, TyCtxtEnsureDone, TyCtxtEnsureOk,
};
pub use self::stack::{QueryStackDeferred, QueryStackFrame, QueryStackFrameExtra};
use crate::dep_graph::{DepNodeIndex, HasDepContext, SerializedDepNodeIndex};
pub use crate::queries::Providers;
use crate::ty::TyCtxt;

pub(crate) mod arena_cached;
mod caches;
pub mod erase;
pub(crate) mod inner;
mod job;
mod keys;
pub mod on_disk_cache;
#[macro_use]
pub mod plumbing;
pub(crate) mod modifiers;
mod stack;

pub fn describe_as_module(def_id: impl Into<LocalDefId>, tcx: TyCtxt<'_>) -> String {
    let def_id = def_id.into();
    if def_id.is_top_level_module() {
        "top-level module".to_string()
    } else {
        format!("module `{}`", tcx.def_path_str(def_id))
    }
}

pub trait QueryContext<'tcx>: HasDepContext {
    /// Gets a jobserver reference which is used to release then acquire
    /// a token while waiting on a query.
    fn jobserver_proxy(&self) -> &Proxy;

    /// Load a side effect associated to the node in the previous session.
    fn load_side_effect(
        self,
        prev_dep_node_index: SerializedDepNodeIndex,
    ) -> Option<QuerySideEffect>;

    /// Register a side effect for the given node, for use in next session.
    fn store_side_effect(self, dep_node_index: DepNodeIndex, side_effect: QuerySideEffect);
}
