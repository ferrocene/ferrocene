//@ compile-flags: --cfg broken

#![crate_type = "lib"]
#![cfg_attr(broken, no_std, no_core)]
//~^ ERROR the `#[no_core]` attribute is an experimental feature

pub struct S {}

// ferrocene-annotations: fls_dd9xh3wdjudo
// Attribute cfg_attr
//
// ferrocene-annotations: um_rustc_cfg
