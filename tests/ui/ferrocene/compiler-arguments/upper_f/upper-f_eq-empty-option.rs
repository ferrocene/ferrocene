// The option of -F starts immediately, without requiring any equal symbol
//
//@ check-pass
//@ compile-flags: -F=
//@ error-pattern: unknown lint

fn main() {}

// ferrocene-annotations: um_rustc_F
