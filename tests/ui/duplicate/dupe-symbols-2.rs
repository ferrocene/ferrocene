//@ build-fail

//
#![crate_type="rlib"]
#![allow(warnings)]

pub mod a {
    #[no_mangle]
    pub extern "C" fn fail() {
    }
}

pub mod b {
    #[no_mangle]
    pub extern "C" fn fail() {
    //~^ symbol `fail` is already defined
    }
}

// ferrocene-annotations: fls_osd6c4utyjb3
// FFI
//
// ferrocene-annotations: fls_pgp7ezcc9lh8
// Foreign Function Interface Attributes
