//@ known-bug: #110395
// FIXME(effects) check-pass

#![feature(derive_const)]
#![feature(const_trait_impl, effects)]

#[derive_const(PartialEq)]
pub struct Reverse<T>(T);

const fn foo(a: Reverse<i32>, b: Reverse<i32>) -> bool {
    a == b
}

fn main() {}
