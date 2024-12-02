//@ run-pass
#![allow(dead_code)]
// Issue #976



fn f<T>(x: Box<T>) {
    let _x2 = x;
}
pub fn main() { }

// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
