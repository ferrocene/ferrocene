//@ run-pass
#![allow(unused_variables)]
// Parsing patterns with paths with type parameters (issue #22544)

use std::default::Default;

#[derive(Default)]
pub struct Foo<T>(T, T);

impl<T: ::std::fmt::Display> Foo<T> {
    fn foo(&self) {
        match *self {
            Foo::<T>(ref x, ref y) => println!("Goodbye, World! {} {}", x, y)
        }
    }
}

trait Tr { //~ WARN trait `Tr` is never used
    type U;
}

impl<T> Tr for Foo<T> {
    type U = T;
}

struct Wrapper<T> {
    value: T
}

fn main() {
    let Foo::<i32>(a, b) = Default::default();

    let f = Foo(2,3);
    f.foo();

    let w = Wrapper { value: Foo(10u8, 11u8) };
    match w {
        Wrapper::<Foo<u8>> { value: Foo(10, 11) } => {},
        ::Wrapper::<<Foo<_> as Tr>::U> { value: Foo::<u8>(11, 16) } => { panic!() },
        _ => { panic!() }
    }

    if let None::<u8> = Some(8) {
        panic!();
    }
    if let None::<u8> { .. } = Some(8) {
        panic!();
    }
    if let Option::None::<u8> { .. } = Some(8) {
        panic!();
    }
}

// ferrocene-annotations: fls_d44aflefat88
// Path Pattern Matching
//
// ferrocene-annotations: fls_uloyjbaso8pz
// Path Patterns
//
// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
//
// ferrocene-annotations: fls_asj8rgccvkoe
// Struct Pattern Matching
//
// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
//
// ferrocene-annotations: fls_eexupzdsu7f
// Tuple Struct Pattern Matching
//
// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
