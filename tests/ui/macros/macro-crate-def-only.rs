//@ run-pass
//@ aux-build:macro_crate_def_only.rs

#![warn(no_link_attributes)]

#[macro_use] #[no_link]
//~^ WARN use of deprecated `no_link` attribute
//~| WARN this was previously accepted by the compiler
extern crate macro_crate_def_only;

pub fn main() {
    assert_eq!(5, make_a_5!());
}

// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
//
// ferrocene-annotations: fls_ch9nkxkloozv
// Attribute no_link
