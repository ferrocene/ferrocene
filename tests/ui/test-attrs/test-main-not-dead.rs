//@ run-pass
//@ compile-flags: --test

#![deny(dead_code)]

fn main() { panic!(); }

// ferrocene-annotations: um_rustc_test
