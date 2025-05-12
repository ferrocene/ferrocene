//@ check-fail
//@ compile-flags: -Cextra-filename
//~? ERROR codegen option `extra-filename` requires a string

fn main() {}

// ferrocene-annotations: um_rustc_C_extra_filename
