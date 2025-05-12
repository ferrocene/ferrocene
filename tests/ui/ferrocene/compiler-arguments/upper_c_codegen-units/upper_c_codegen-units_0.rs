//@ check-fail
//@ compile-flags: -Ccodegen-units=0
//~? ERROR value for codegen units must be a positive non-zero integer

fn main() {}

// ferrocene-annotations: um_rustc_C_codegen_units
