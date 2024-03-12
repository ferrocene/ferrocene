//@ run-pass
//@ compile-flags: -D warnings -D unknown-lints

#![allow(unknown_lints)]
#![allow(random_lint_name)]

fn main() {}

// ferrocene-annotations: um_rustc_D
