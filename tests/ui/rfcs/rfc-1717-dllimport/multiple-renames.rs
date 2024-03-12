//@ compile-flags: -l foo:bar -l foo:baz
//@ error-pattern: multiple renamings were specified for library

#![crate_type = "lib"]

#[link(name = "foo")]
extern "C" {}

// ferrocene-annotations: fls_o0f9ae22ug1x
// Attribute link
//
// ferrocene-annotations: um_rustc_l
