//@ When both -F and -D are specified for the same lint, the lint is denied.
//
//@ check-fail
//@ compile-flags: -D missing_docs -F overflowing_literals

fn main() {} //~ ERROR

// ferrocene-annotations: um_rustc_F
