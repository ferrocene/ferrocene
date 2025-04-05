//@ build-fail
//@ ignore-wasi wasi does different things with the `main` symbol

#![allow(warnings)]

#[no_mangle]
fn main(){} //~ ERROR entry symbol `main` declared multiple times

// ferrocene-annotations: fls_osd6c4utyjb3
// FFI
//
// ferrocene-annotations: fls_pgp7ezcc9lh8
// Foreign Function Interface Attributes
