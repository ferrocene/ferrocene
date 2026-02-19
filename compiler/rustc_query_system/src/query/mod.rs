use std::fmt::Debug;

use rustc_errors::DiagInner;
use rustc_macros::{Decodable, Encodable};

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
