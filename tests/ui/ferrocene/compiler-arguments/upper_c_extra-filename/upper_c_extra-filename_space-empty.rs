//@ check-fail
//@ compile-flags: -C extra-filename
//~? ERROR codegen option `extra-filename` requires a string

fn main() {}

// ferrocene-annotations: um_rustc_C_extra_filename
