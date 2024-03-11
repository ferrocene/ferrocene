//@ run-pass
#![allow(unused_assignments)]

pub fn main() {
    let i: Box<_> = Box::new(1);
    let mut j: Box<_> = Box::new(2);
    // Should drop the previous value of j
    j = i;
    assert_eq!(*j, 1);
}

// ferrocene-annotations: fls_77scxuomlbgs
// Passing Conventions
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
//
// ferrocene-annotations: fls_y4by2i8dl05o
// Assignment Expressions
//
// ferrocene-annotations: fls_nnqlae9zp80s
// Basic Assignment
//
// ferrocene-annotations: fls_4jiw35pan7vn
// Destruction
//
// ferrocene-annotations: fls_u2mzjgiwbkz0
// Destructors
//
// ferrocene-annotations: fls_rm4ncoopcdvj
// Drop Scopes
//
// ferrocene-annotations: fls_5eima0pd31c0
// Drop Scope Extension
//
// ferrocene-annotations: fls_afafmafz4hf2
// Drop Order
