//@ check-pass

#![allow(bare_trait_objects)]

type A = Box<(Fn(u8) -> u8) + 'static + Send + Sync>; // OK (but see #39318)

fn main() {}

// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
