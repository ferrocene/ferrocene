//@ run-pass

#![allow(unused_variables)]

pub fn main() {
    let i: Box<_> = Box::new(100);
    let j: Box<_> = Box::new(200);
    let j = i;
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
