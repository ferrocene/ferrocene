//@ This test should check what happen when no option is provided to -o
//@ Unfortunately, the test frameworking is adding other options after it,
//@ causing an unexpected failure.
//
//@ check-fail
//@ compile-flags: -o

fn main() {}

// ferrocene-annotations: um_rustc_o
