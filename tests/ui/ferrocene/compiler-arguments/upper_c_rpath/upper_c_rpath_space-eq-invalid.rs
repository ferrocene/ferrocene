//@ check-fail
//@ compile-flags: -C rpath=invalid
//~? ERROR incorrect value

fn main() {}

// ferrocene-annotations: um_rustc_C_rpath
