//@ check-fail
//@ compile-flags: --target=x86_64-invalid-linux-gnu
//@ needs-llvm-components:
//~? Could not find specification for target

fn main() {}

// ferrocene-annotations: um_rustc_target
