// When -F is followed by a valid option, the lint is denied
//
//@ check-fail
//@ compile-flags: -F missing_docs

fn main() {} //~ ERROR

// ferrocene-annotations: um_rustc_F
