//@ check-fail
//@ compile-flags: -Ccodegen-units=-1
//~? ERROR incorrect value

fn main() {}

// ferrocene-annotations: um_rustc_C_codegen_units
