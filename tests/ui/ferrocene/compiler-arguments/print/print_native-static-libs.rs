//@ check-pass
//@ compile-flags: --print native-static-libs
//@ only-x86_64-unknown-linux-gnu
//~? WARN cannot output linkage information without staticlib crate-type

fn main() {}

// ferrocene-annotations: um_rustc_print
