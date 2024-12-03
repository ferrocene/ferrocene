//@ run-pass

#![allow(dead_code)]

#[derive(Clone)]
struct S<T> {
    foo: (),
    bar: (),
    baz: T,
}

pub fn main() {
    let _ = S { foo: (), bar: (), baz: 1 }.clone();
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Type
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
