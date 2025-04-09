// The option of -A starts immediately, without requiring any equal symbol
//
//@ check-pass
//@ compile-flags: -A=
//~? unknown lint
//~? unknown lint
//~? unknown lint

fn main() {}

// ferrocene-annotations: um_rustc_A
