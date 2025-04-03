//@ check-fail
//@ compile-flags: -Cpanic
//~? requires either `unwind` or `abort`

fn main() {}

// ferrocene-annotations: um_rustc_C_panic
