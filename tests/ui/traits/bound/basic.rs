//@ run-pass
#![allow(dead_code)]
#![allow(unconditional_recursion)]


trait Foo {
}

fn b(_x: Box<dyn Foo+Send>) {
}

fn c(x: Box<dyn Foo+Sync+Send>) {
    e(x);
}

fn d(x: Box<dyn Foo+Send>) {
    e(x);
}

fn e(x: Box<dyn Foo>) {
    e(x);
}

pub fn main() { }

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_ikfvbeewame7
// Subtyping and Variance
//
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
