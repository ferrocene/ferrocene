//@ run-pass
#![allow(non_camel_case_types)]

//@ aux-build:namespaced_enum_emulate_flat.rs


extern crate namespaced_enum_emulate_flat;

use namespaced_enum_emulate_flat::{Foo, A, B, C};
use namespaced_enum_emulate_flat::nest::{Bar, D, E, F};

fn _f(f: Foo) {
    match f {
        A | B(_) | C { .. } => {}
    }
}

fn _f2(f: Bar) {
    match f {
        D | E(_) | F { .. } => {}
    }
}

pub fn main() {}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_dq403wq5yrs
// Namespaces
