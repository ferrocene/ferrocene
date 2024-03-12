//@ run-pass
#![allow(dead_code)]
trait Trait<T> {
    fn f(&self, x: T);
}

#[derive(Copy, Clone)]
struct Struct {
    x: isize,
    y: isize,
}

impl Trait<&'static str> for Struct {
    fn f(&self, x: &'static str) {
        println!("Hi, {}!", x);
    }
}

pub fn main() {
    let a = Struct { x: 1, y: 2 };
    let b: Box<dyn Trait<&'static str>> = Box::new(a);
    b.f("Mary");
    let c: &dyn Trait<&'static str> = &a;
    c.f("Joe");
}

// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
