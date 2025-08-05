// Regression test for <https://github.com/rust-lang/rust/pull/113374> to
// ensure it doesn't panic.

//@ edition: 2015

mod generics {
    pub enum WherePredicate {
        EqPredicate,
    }
}
pub mod visit {
    use *;
    pub fn visit_where_predicate<V>(_visitor: &mut V, _i: &WherePredicate) {}
}
pub use generics::*;
