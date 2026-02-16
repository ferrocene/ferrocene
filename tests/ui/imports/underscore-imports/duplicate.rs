//@ check-pass
//@ proc-macro: duplicate.rs

extern crate duplicate;

#[duplicate::duplicate]
use main as _; // OK

macro_rules! duplicate {
    ($item: item) => { $item $item }
}

duplicate!(use std as _;); // OK

fn main() {}

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
//
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
