// Test --extern with an invalid crate name. Surprisingly, it's accepted.
//@ check-fail
//@ aux-build:some_crate.rs
//@ compile-flags:--extern -
//@ edition:2018

fn main() {}

// ferrocene-annotations: um_rustc_extern
