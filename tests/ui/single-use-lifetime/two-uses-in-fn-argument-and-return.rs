// Test that we DO NOT warn when lifetime name is used in
// both the argument and return.
//
//@ check-pass

#![deny(single_use_lifetimes)]
#![allow(dead_code)]
#![allow(unused_variables)]

// OK: used twice
fn c<'a>(x: &'a u32) -> &'a u32 {
    &22
}

fn main() {}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_wttihxen35as
// Constant Promotion
