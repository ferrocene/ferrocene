//@ run-pass
use std::cmp::Ordering::{Less,Equal,Greater};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct A<'a> {
    x: &'a isize
}
pub fn main() {
    let (a, b) = (A { x: &1 }, A { x: &2 });

    assert_eq!(a.cmp(&a), Equal);
    assert_eq!(b.cmp(&b), Equal);

    assert_eq!(a.cmp(&b), Less);
    assert_eq!(b.cmp(&a), Greater);
}

// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetime
//
// ferrocene-annotations: fls_nsvzzbldhq53
// Comparison Expressions
