//@ run-pass
#![allow(dead_code)]

struct S {
    f0: String,
    f1: String,
}

pub fn main() {
    let s = "Hello, world!".to_string();
    let s = S {
        f1: s.to_string(),
        f0: s
    };
    assert_eq!(s.f0, "Hello, world!");
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
