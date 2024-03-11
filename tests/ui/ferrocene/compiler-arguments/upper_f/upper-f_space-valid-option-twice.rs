// -F can be specified multiple times, repeating the same option is not considered an error.
//
//@ check-fail
//@ compile-flags: -F missing_docs -F overflowing_literals

fn main() {} //~ ERROR

// ferrocene-annotations: um_rustc_F
