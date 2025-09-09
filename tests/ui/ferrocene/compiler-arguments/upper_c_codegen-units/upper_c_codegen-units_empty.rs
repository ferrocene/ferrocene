//@ check-fail
//@ compile-flags: -Ccodegen-units
//~? ERROR codegen option `codegen-units` requires a number

fn main() {}

// ferrocene-annotations: um_rustc_C_codegen_units
