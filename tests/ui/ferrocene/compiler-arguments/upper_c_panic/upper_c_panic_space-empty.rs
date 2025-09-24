//@ check-fail
//@ compile-flags: -C panic
//~? ERROR requires either `unwind`, `abort`, or `immediate-abort`

fn main() {}

// ferrocene-annotations: um_rustc_C_panic
