//@ check-pass
//@ aux-build:empty-struct.rs

#![warn(no_link_attributes)]

#[no_link]
//~^ WARN use of deprecated `no_link` attribute
//~| WARN this was previously accepted by the compiler
extern crate empty_struct;

fn main() {
    empty_struct::XEmpty {};
}

// ferrocene-annotations: fls_ch9nkxkloozv
// Attribute no_link
