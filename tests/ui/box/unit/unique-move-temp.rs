//@ run-pass
#![allow(unused_mut)]

pub fn main() {
    let mut i: Box<_>;
    i = Box::new(100);
    assert_eq!(*i, 100);
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
// ferrocene-annotations: fls_cleoffpn5ew6
// Temporaries
//
// ferrocene-annotations: fls_gho955gmob73
// Variables
