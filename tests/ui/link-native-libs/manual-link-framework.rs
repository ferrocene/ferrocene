//@ ignore-apple
//@ compile-flags:-l framework=foo

fn main() {}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_l
=======
//~? ERROR library kind `framework` is only supported on Apple targets
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
