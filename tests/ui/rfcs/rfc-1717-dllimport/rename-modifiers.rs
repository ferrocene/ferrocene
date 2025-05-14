//@ compile-flags: -l dylib=foo:bar

#![feature(native_link_modifiers_as_needed)]

#![crate_type = "lib"]

#[link(name = "foo", kind = "dylib", modifiers = "-as-needed")]
extern "C" {}
//~^ ERROR overriding linking modifiers from command line is not supported

// ferrocene-annotations: fls_o0f9ae22ug1x
// Attribute link
//
// ferrocene-annotations: um_rustc_l
