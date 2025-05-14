//@ check-fail
//@ compile-flags: -C link-arg
//~? ERROR codegen option `link-arg` requires a string

fn main() {}

// ferrocene-annotations: um_rustc_C_link_arg
