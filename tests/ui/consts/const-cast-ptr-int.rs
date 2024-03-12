//@ run-pass
#![allow(non_upper_case_globals)]

use std::ptr;

struct TestStruct {
    x: *const u8
}

unsafe impl Sync for TestStruct {}

static a: TestStruct = TestStruct{x: 0 as *const u8};

pub fn main() {
    assert_eq!(a.x, ptr::null());
}

// ferrocene-annotations: fls_1qhsun1vyarz
// Type cast expressions
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
