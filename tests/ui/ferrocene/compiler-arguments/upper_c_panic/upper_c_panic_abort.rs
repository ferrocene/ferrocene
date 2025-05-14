//@ build-fail
//@ compile-flags: -Cpanic=abort
//@ needs-unwind
//~? ERROR not compiled with this crate's panic strategy

fn main() {}

// ferrocene-annotations: um_rustc_C_panic
