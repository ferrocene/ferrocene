//@ run-pass
#![allow(dead_code)]
//@ pretty-expanded FIXME #23616

#[derive(Hash)]
struct Foo {
    x: isize,
    y: isize,
    z: isize
}

pub fn main() {}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Type
