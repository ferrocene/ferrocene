//@ compile-flags: -l foo:bar -l foo:baz

#![crate_type = "lib"]

#[link(name = "foo")]
extern "C" {}

//~? ERROR multiple renamings were specified for library `foo`

// ferrocene-annotations: fls_o0f9ae22ug1x
// Attribute link
//
// ferrocene-annotations: um_rustc_l
