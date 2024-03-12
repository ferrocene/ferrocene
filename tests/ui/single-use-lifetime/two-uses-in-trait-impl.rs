// Test that we DO NOT warn for a lifetime on an impl used in both
// header and in an associated type.
//
//@ check-pass

#![deny(single_use_lifetimes)]
#![allow(dead_code)]
#![allow(unused_variables)]

struct Foo<'f> {
    data: &'f u32,
}

impl<'f> Iterator for Foo<'f> {
    type Item = &'f u32;

    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

fn main() {}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
