//@ compile-flags: -l foo:bar
//@ error-pattern: renaming of the library `foo` was specified

#![crate_type = "lib"]

// ferrocene-annotations: um_rustc_l
