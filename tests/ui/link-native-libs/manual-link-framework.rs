//@ ignore-apple
//@ compile-flags:-l framework=foo

fn main() {}

//~? ERROR library kind `framework` is only supported on Apple targets

// ferrocene-annotations: um_rustc_l
