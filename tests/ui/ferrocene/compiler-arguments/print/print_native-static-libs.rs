//@ check-pass
//@ compile-flags: --print native-static-libs
//~? WARN cannot output linkage information without staticlib crate-type

fn main() {}

// ferrocene-annotations: um_rustc_print
