//@ run-pass
#![allow(dead_code)]

struct S { f0: String, f1: isize }

pub fn main() {
    let s = "Hello, world!".to_string();
    let s = S {
        f0: s.to_string(),
        ..S {
            f0: s,
            f1: 23
        }
    };
    assert_eq!(s.f0, "Hello, world!");
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
