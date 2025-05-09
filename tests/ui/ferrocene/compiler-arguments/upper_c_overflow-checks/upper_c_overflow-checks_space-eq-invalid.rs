//@ check-fail
//@ compile-flags: -C overflow-checks=invalid
//~? ERROR incorrect value

fn main() {}

// ferrocene-annotations: um_rustc_C_overflow_checks
