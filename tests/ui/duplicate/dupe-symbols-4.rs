//@ build-fail

#![crate_type="rlib"]
#![allow(warnings)]


pub trait A {
    fn fail(self);
}

struct B;
struct C;

impl A for B {
    #[no_mangle]
    fn fail(self) {}
}

impl A for C {
    #[no_mangle]
    fn fail(self) {} //~ ERROR symbol `fail` is already defined
}

// ferrocene-annotations: fls_osd6c4utyjb3
// FFI
//
// ferrocene-annotations: fls_pgp7ezcc9lh8
// Foreign Function Interface Attributes
