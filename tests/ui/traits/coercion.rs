//@ run-pass
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_variables)]

use std::io::{self, Write};

trait Trait {
    fn f(&self);
}

#[derive(Copy, Clone)]
struct Struct {
    x: isize,
    y: isize,
}

impl Trait for Struct {
    fn f(&self) {
        println!("Hi!");
    }
}

fn foo(mut a: Box<dyn Write>) {}

pub fn main() {
    let a = Struct { x: 1, y: 2 };
    let b: Box<dyn Trait> = Box::new(a);
    b.f();
    let c: &dyn Trait = &a;
    c.f();

    let out = io::stdout();
    foo(Box::new(out));
}

// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
