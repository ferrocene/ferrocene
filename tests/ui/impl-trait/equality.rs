//@ dont-require-annotations: NOTE

#![feature(specialization)] //~ WARN the feature `specialization` is incomplete

trait Foo: Copy + ToString {}

impl<T: Copy + ToString> Foo for T {}

fn hide<T: Foo>(x: T) -> impl Foo {
    x
}

fn two(x: bool) -> impl Foo {
    if x {
        return 1_i32;
    }
    0_u32
    //~^ ERROR mismatched types
    //~| NOTE expected `i32`, found `u32`
}

fn sum_to(n: u32) -> impl Foo {
    if n == 0 {
        0
    } else {
        n + sum_to(n - 1)
        //~^ ERROR cannot add `impl Foo` to `u32`
    }
}

trait Leak: Sized {
    type T;
    fn leak(self) -> Self::T;
}
impl<T> Leak for T {
    default type T = ();
    default fn leak(self) -> Self::T { panic!() }
}
impl Leak for i32 {
    type T = i32;
    fn leak(self) -> i32 { self }
}

fn main() {
}

// ferrocene-annotations: fls_46ork6fz5o2e
// Implementation Coherence
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
