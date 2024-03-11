//@ check-pass
//@ edition:2018

// This test is similar to `ambiguity.rs`, but nested in a module.

#![allow(non_camel_case_types)]

mod foo {
    pub use std::io; // OK

    mod std {
        pub struct io;
    }
}

fn main() {}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
