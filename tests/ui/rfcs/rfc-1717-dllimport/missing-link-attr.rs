//@ compile-flags: -l foo:bar

#![crate_type = "lib"]

//~? ERROR renaming of the library `foo` was specified

// ferrocene-annotations: um_rustc_l
