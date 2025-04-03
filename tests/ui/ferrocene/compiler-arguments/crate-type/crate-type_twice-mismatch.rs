//@ check-fail
//@ compile-flags: --crate-type=bin --crate-type=lib
//~? cannot mix `bin` crate type with others

fn main() {}

// ferrocene-annotations: um_rustc_crate_type
