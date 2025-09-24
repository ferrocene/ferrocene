//@ compile-flags:-C panic=foo

fn main() {}

//~? ERROR incorrect value `foo` for codegen option `panic` - either `unwind`, `abort`, or `immediate-abort` was expected

// ferrocene-annotations: um_rustc_C_panic
