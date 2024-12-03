//@ run-pass
//@ proc-macro: multiple-derives.rs

extern crate multiple_derives;

use multiple_derives::*;

#[derive(First)]
#[derive(Second)]
#[derive(Third, Fourth)]
#[derive(Fifth)]
pub struct Foo {}

fn main() {}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
