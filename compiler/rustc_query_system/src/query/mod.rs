use std::fmt::Debug;

use rustc_data_structures::jobserver::Proxy;
use rustc_errors::DiagInner;
use rustc_macros::{Decodable, Encodable};

pub use self::plumbing::*;
use crate::dep_graph::{DepNodeIndex, HasDepContext, SerializedDepNodeIndex};

mod plumbing;

/// How a particular query deals with query cycle errors.
///
/// Inspected by the code that actually handles cycle errors, to decide what
/// approach to use.
#[derive(Copy, Clone)]
pub enum CycleErrorHandling {
    Error,
    Fatal,
    DelayBug,
    Stash,
}

/// Tracks 'side effects' for a particular query.
/// This struct is saved to disk along with the query result,
/// and loaded from disk if we mark the query as green.
/// This allows us to 'replay' changes to global state
/// that would otherwise only occur if we actually
/// executed the query method.
///
/// Each side effect gets an unique dep node index which is added
/// as a dependency of the query which had the effect.
#[derive(Debug, Encodable, Decodable)]
pub enum QuerySideEffect {
    /// Stores a diagnostic emitted during query execution.
    /// This diagnostic will be re-emitted if we mark
    /// the query as green, as that query will have the side
    /// effect dep node as a dependency.
    Diagnostic(DiagInner),
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
