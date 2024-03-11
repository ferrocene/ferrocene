//@ compile-flags: -l foo:
//@ error-pattern: an empty renaming target was specified for library

#![crate_type = "lib"]

#[link(name = "foo")]
extern "C" {}

// ferrocene-annotations: fls_o0f9ae22ug1x
// Attribute link
//
// ferrocene-annotations: um_rustc_l
