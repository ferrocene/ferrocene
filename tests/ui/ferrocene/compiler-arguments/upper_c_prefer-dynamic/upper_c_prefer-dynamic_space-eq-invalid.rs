//@ check-fail
//@ compile-flags: -C prefer-dynamic=invalid
//~? ERROR incorrect value

fn main() {}

// ferrocene-annotations: um_rustc_C_prefer_dynamic
