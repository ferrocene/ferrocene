// Unspecified kind should fail with an error

//@ compile-flags: -l :+bundle=mylib

fn main() {}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_l
=======
//~? ERROR unknown library kind ``
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
