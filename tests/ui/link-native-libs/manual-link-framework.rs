//@ ignore-apple
//@ compile-flags:-l framework=foo
//@ error-pattern: library kind `framework` is only supported on Apple targets

fn main() {}

// ferrocene-annotations: um_rustc_l
