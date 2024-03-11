//@ compile-flags:-C panic=foo
//@ error-pattern:either `unwind` or `abort` was expected

fn main() {}

// ferrocene-annotations: um_rustc_C_panic
