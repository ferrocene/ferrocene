// -A must be followed by a valid lint option
//
//@ check-pass
//@ compile-flags: -A invalid_lint_code
//~? WARN unknown lint
//~? WARN unknown lint
//~? WARN unknown lint

fn main() {}

// ferrocene-annotations: um_rustc_A
