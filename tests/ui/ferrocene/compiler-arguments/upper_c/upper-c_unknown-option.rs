//@ check-fail
//@ compile-flags: -C this-codegen-option-does-not-exist
//~? unknown codegen option

fn main() {}

// ferrocene-annotations: um_rustc_C
