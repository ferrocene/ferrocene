//@ run-pass
//@ pretty-expanded FIXME #23616

#![allow(dead_code)]

#[derive(Clone)]
struct S((), ());

pub fn main() {}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Type
