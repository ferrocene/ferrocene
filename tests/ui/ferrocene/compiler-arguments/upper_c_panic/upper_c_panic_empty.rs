//@ check-fail
//@ compile-flags: -Cpanic
//~? ERROR requires either `unwind`, `abort`, or `immediate-abort`

fn main() {}

// ferrocene-annotations: um_rustc_C_panic
