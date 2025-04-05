//@ compile-flags: -l foo:bar

#![crate_type = "lib"]

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_l
=======
//~? ERROR renaming of the library `foo` was specified
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
