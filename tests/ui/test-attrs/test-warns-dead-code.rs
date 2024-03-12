//@ compile-flags: --test

#![deny(dead_code)]

fn dead() {} //~ error: function `dead` is never used

fn main() {}

// ferrocene-annotations: um_rustc_test
