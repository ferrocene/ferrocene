//@ run-pass
#![allow(unused_mut)]

pub fn main() {
    let i: Box<_> = Box::new(100);
    let mut j;
    j = i;
    assert_eq!(*j, 100);
}

// ferrocene-annotations: fls_svkx6szhr472
// Ownership
//
// ferrocene-annotations: fls_4jiw35pan7vn
// Destruction
//
// ferrocene-annotations: fls_u2mzjgiwbkz0
// Destructors
//
// ferrocene-annotations: fls_4jiw35pan7vn
// Destruction
//
// ferrocene-annotations: fls_u2mzjgiwbkz0
// Destructors
//
// ferrocene-annotations: fls_y4by2i8dl05o
// Assignment Expressions
//
// ferrocene-annotations: fls_nnqlae9zp80s
// Basic Assignment
