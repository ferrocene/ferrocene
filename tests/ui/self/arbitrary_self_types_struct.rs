//@ run-pass

use std::rc::Rc;

struct Foo {
    x: i32,
    y: i32,
}

impl Foo {
    fn x(self: &Rc<Self>) -> i32 {
        self.x
    }

    fn y(self: Rc<Self>) -> i32 {
        self.y
    }
}

fn main() {
    let foo = Rc::new(Foo {x: 3, y: 4});
    assert_eq!(3, foo.x());
    assert_eq!(4, foo.y());
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
