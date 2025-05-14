// The option of -A starts immediately, without requiring any equal symbol
//
//@ check-pass
//@ compile-flags: -A=
//~? WARN unknown lint
//~? WARN unknown lint
//~? WARN unknown lint

fn main() {}

// ferrocene-annotations: um_rustc_A
