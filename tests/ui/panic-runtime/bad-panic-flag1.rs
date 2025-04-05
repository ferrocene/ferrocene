//@ compile-flags:-C panic=foo

fn main() {}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_C_panic
=======
//~? ERROR incorrect value `foo` for codegen option `panic` - either `unwind` or `abort` was expected
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
