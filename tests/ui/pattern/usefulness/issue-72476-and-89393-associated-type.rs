//@ check-pass

// From https://github.com/rust-lang/rust/issues/72476
// and https://github.com/rust-lang/rust/issues/89393

trait Trait {
    type Projection;
}

struct A;
impl Trait for A {
    type Projection = bool;
}

struct B;
impl Trait for B {
    type Projection = (u32, u32);
}

struct Next<T: Trait>(T::Projection);

fn foo1(item: Next<A>) {
    match item {
        Next(true) => {}
        Next(false) => {}
    }
}

fn foo2(x: <A as Trait>::Projection) {
    match x {
        true => {}
        false => {}
    }
}

fn foo3(x: Next<B>) {
    let Next((_, _)) = x;
    match x {
        Next((_, _)) => {}
    }
}

fn foo4(x: <B as Trait>::Projection) {
    let (_, _) = x;
    match x {
        (_, _) => {}
    }
}

fn foo5<T: Trait>(x: <T as Trait>::Projection) {
    match x {
        _ => {}
    }
}

fn main() {}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Underscore Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Underscore Pattern Matching
