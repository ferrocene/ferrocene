//@ run-pass

#![allow(dead_code)]
// Tests that even when a type parameter doesn't implement a required
// super-builtin-kind of a trait, if the type parameter is never used,
// the type can implement the trait anyway.


use std::marker;

trait Foo : Send { }

struct X<T> { marker: marker::PhantomData<T> }

impl<T:Send> Foo for X<T> { }

pub fn main() { }

// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_eiw4by8z75di
// Send and Sync
