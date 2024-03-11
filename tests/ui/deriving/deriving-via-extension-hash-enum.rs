//@ run-pass
#![allow(dead_code)]
#[derive(Hash)]
enum Foo {
    Bar(isize, char),
    Baz(char, isize)
}

#[derive(Hash)]
enum A {
    B,
    C,
    D,
    E
}

pub fn main(){}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Type
