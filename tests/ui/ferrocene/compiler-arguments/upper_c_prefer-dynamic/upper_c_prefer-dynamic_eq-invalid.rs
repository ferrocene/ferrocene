//@ check-fail
//@ compile-flags: -Cprefer-dynamic=invalid
//~? ERROR incorrect value

fn main() {}

// ferrocene-annotations: um_rustc_C_prefer_dynamic
