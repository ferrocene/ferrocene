//@ check-fail
//@ compile-flags: -C panic
//~? ERROR requires either `unwind` or `abort`

fn main() {}

// ferrocene-annotations: um_rustc_C_panic
