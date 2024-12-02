//@ run-pass
#![allow(dead_code)]

enum Foo {
    Bar {
        a: isize,
        b: isize
    },
    Baz {
        c: f64,
        d: f64
    }
}

pub fn main() {
    let _x = Foo::Bar { a: 2, b: 3 };
}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
