// When both -F and -A are specified for the same lint, the last one does not override the
// previous. So here, -A does not override -F.
//
//@ check-fail
//@ compile-flags: -F missing_docs -A overflowing_literals

fn main() {} //~ ERROR

// ferrocene-annotations: um_rustc_F
