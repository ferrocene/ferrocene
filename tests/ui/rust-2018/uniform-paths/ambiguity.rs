//@ check-pass
//@ edition:2018

#![allow(non_camel_case_types)]

use std::io; // OK

mod std {
    pub struct io;
}

fn main() {}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
