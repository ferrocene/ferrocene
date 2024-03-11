//@ build-pass (FIXME(62277): could be check-pass?)
//@ aux-build:test_macro.rs
//@ compile-flags:--test

#[macro_use] extern crate test_macro;

#[test]
fn foo(){}

macro_rules! test { () => () }

#[test]
fn bar() {}

// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
//
// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: um_rustc_test
