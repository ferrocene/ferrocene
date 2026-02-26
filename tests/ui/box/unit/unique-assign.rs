<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
//@ run-pass
#![allow(unused_mut)]

pub fn main() {
    let mut i: Box<_>;
    i = Box::new(1);
    assert_eq!(*i, 1);
}

// ferrocene-annotations: fls_y4by2i8dl05o
// Assignment Expressions
//
// ferrocene-annotations: fls_nnqlae9zp80s
// Basic Assignment
