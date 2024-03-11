//@ aux-build:uninhabited.rs
#![deny(unreachable_patterns)]
#![feature(never_type)]

extern crate uninhabited;

use uninhabited::PartiallyInhabitedVariants;

// This test checks a redundant/useless pattern of a non-exhaustive enum/variant is still
// warned against.

pub fn foo(x: PartiallyInhabitedVariants) {
    match x {
        PartiallyInhabitedVariants::Struct { .. } => {},
        PartiallyInhabitedVariants::Struct { .. } => {},
        //~^ ERROR unreachable pattern
        _ => {},
    }
}

fn main() { }

// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
// ferrocene-annotations: fls_98lnexk53ru4
// Never Type
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
