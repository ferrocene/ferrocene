//@ edition: 2015
//@ check-pass

#![allow(bare_trait_objects)]

use std::fmt::Debug;

fn main() {
    let x: Box<Debug+> = Box::new(3) as Box<Debug+>; // Trailing `+` is OK
}

// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
