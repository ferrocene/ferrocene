//@ check-pass
#![feature(generic_const_exprs)]
#![allow(incomplete_features)]

use std::mem::size_of;

struct Foo<T, const N: usize>(T);

impl<T> Foo<T, { size_of::<T>() }> {
    fn test() {
        let _: [u8; std::mem::size_of::<T>()];
    }
}

trait Bar<const N: usize> {
    fn test_me();
}

impl<T> Bar<{ size_of::<T>() }> for Foo<T, 3> {
    fn test_me() {
        let _: [u8; std::mem::size_of::<T>()];
    }
}

fn main() {}

// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation conformance
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
