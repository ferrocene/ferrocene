//@ check-fail
//@ compile-flags: --target=x86_64-invalid-linux-gnu
//@ needs-llvm-components: x86
//~? ERROR error loading target specification: could not find specification for target "x86_64-invalid-linux-gnu"

fn main() {}

// ferrocene-annotations: um_rustc_target
