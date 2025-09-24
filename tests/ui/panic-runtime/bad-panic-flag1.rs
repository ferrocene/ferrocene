//@ compile-flags:-C panic=foo

fn main() {}

<<<<<<< HEAD
//~? ERROR incorrect value `foo` for codegen option `panic` - either `unwind` or `abort` was expected

// ferrocene-annotations: um_rustc_C_panic
=======
//~? ERROR incorrect value `foo` for codegen option `panic` - either `unwind`, `abort`, or `immediate-abort` was expected
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
