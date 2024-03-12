// Test that the borrow checker considers `#[non_exhaustive]` when checking
// whether a match contains a discriminant read.

//@ aux-build:monovariants.rs
extern crate monovariants;

use monovariants::NonExhaustiveMonovariant;

fn main() {
    let mut x = NonExhaustiveMonovariant::Variant(1);
    let y = &mut x;
    match x {
        //~^ ERROR cannot use `x` because it was mutably borrowed
        NonExhaustiveMonovariant::Variant(_) => {},
        _ => {},
    }
    drop(y);
}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
// ferrocene-annotations: fls_eexupzdsu7f
// Tuple Struct Pattern Matching
