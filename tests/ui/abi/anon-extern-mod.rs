//@ run-pass
//@ pretty-expanded FIXME #23616

#![feature(rustc_private)]

extern crate libc;

#[link(name = "rust_test_helpers", kind = "static")]
extern "C" {
    fn rust_get_test_int() -> libc::intptr_t;
}

pub fn main() {
    unsafe {
        let _ = rust_get_test_int();
    }
}
