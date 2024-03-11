//@ build-fail

//
//@ error-pattern: entry symbol `main` declared multiple times

#![allow(warnings)]

#[no_mangle]
fn main(){}

// ferrocene-annotations: fls_osd6c4utyjb3
// FFI
//
// ferrocene-annotations: fls_pgp7ezcc9lh8
// Foreign Function Interface Attributes
