// -F must be followed by a valid lint option
//
//@ check-pass
//@ compile-flags: -F invalid_lint_code
//@ error-pattern: unknown lint

fn main() {}

// ferrocene-annotations: um_rustc_F
