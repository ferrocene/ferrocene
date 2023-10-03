extern crate core;

use core::ffi::c_int;

extern "C" {
    fn plus (a: c_int, b: c_int) -> c_int;
}

fn main() {
    assert!(unsafe { plus(1, 2) } == 3);
}
