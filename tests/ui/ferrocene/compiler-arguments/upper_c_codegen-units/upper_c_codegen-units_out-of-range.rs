//@ check-fail
//@ compile-flags: -Ccodegen-units=18446744073709551616
//~? ERROR incorrect value

fn main() {}

// ferrocene-annotations: um_rustc_C_codegen_units
