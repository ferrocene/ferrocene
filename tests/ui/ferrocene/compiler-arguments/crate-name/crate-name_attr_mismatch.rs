//@ check-fail
//@ compile-flags: --crate-name=foo

#![crate_name = "bar"] //~ ERROR
fn main() {}

// ferrocene-annotations: um_rustc_crate_name
