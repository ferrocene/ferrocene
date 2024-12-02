//@ run-pass
#![allow(dead_code)]
#![allow(non_camel_case_types)]

// Passing enums by value


pub enum void {}

mod bindgen {
    use super::void;

    extern "C" {
        pub fn printf(v: void);
    }
}

pub fn main() {}

// ferrocene-annotations: fls_usgd0xlijoxv
// ABI
//
// ferrocene-annotations: fls_osd6c4utyjb3
// FFI
