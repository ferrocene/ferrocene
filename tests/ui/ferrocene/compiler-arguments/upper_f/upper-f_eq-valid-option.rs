// If a valid option is set for -F, but an equal symbol is used as a delimiter,
// we have a failure and a hint.
//
//@ check-pass
//@ compile-flags: -F=missing_docs
//~? unknown lint
//~? unknown lint
//~? unknown lint

fn main() {}

// ferrocene-annotations: um_rustc_F
