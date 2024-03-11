//@ build-pass
#![allow(dead_code)]

struct A {
    a: &'static (),
}

static B: &'static A = &A { a: &() };
static C: &'static A = &B;

fn main() {}

// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
