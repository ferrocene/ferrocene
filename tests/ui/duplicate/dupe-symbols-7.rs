//@ build-fail
//@ ignore-wasi wasi does different things with the `main` symbol

#![allow(warnings)]

#[no_mangle]
<<<<<<< HEAD
fn main(){}

// ferrocene-annotations: fls_osd6c4utyjb3
// FFI
//
// ferrocene-annotations: fls_pgp7ezcc9lh8
// Foreign Function Interface Attributes
=======
fn main(){} //~ ERROR entry symbol `main` declared multiple times
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
