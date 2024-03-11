//@ run-pass
#![allow(unused_allocation)]

use std::rc::Rc;

trait Trait {
    fn trait_method<'a>(self: &'a Box<Rc<Self>>) -> &'a [i32];
}

impl Trait for Vec<i32> {
    fn trait_method<'a>(self: &'a Box<Rc<Self>>) -> &'a [i32] {
        &***self
    }
}

fn main() {
    let v = vec![1, 2, 3];

    assert_eq!(&[1, 2, 3], Box::new(Rc::new(v)).trait_method());
}

// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
//
// ferrocene-annotations: fls_xinykul167l
// Array Expressions
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
