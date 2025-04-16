// See https://github.com/rust-lang/rust/issues/88470
//@ run-rustfix
//@ edition:2018
//@ check-pass
#![warn(rust_2021_prelude_collisions)]
#![allow(dead_code)]
#![allow(unused_imports)]

pub trait PyTryFrom<'v, T>: Sized {
    fn try_from<V>(value: V) -> Result<&'v Self, T>;
}

pub trait PyTryInto<T>: Sized {
    fn try_into(&self) -> Result<&T, i32>;
}

struct Foo;

impl<U> PyTryInto<U> for Foo
where
    U: for<'v> PyTryFrom<'v, i32>,
{
    fn try_into(&self) -> Result<&U, i32> {
        U::try_from(self)
        //~^ WARNING trait-associated function `try_from` will become ambiguous in Rust 2021
        //~| WARN this is accepted in the current edition (Rust 2018)
    }
}

fn main() {}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
//
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
