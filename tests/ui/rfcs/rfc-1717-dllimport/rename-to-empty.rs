//@ compile-flags: -l foo:

#![crate_type = "lib"]

#[link(name = "foo")]
extern "C" {}

//~? ERROR an empty renaming target was specified for library `foo`

// ferrocene-annotations: fls_o0f9ae22ug1x
// Attribute link
//
// ferrocene-annotations: um_rustc_l
