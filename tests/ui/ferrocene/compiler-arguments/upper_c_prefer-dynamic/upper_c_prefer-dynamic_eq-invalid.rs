//@ check-fail
//@ compile-flags: -Cprefer-dynamic=invalid
//~? incorrect value

fn main() {}

// ferrocene-annotations: um_rustc_C_prefer_dynamic
