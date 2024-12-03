//@ run-pass
#![allow(unused_variables)]

struct Bencher;

// ICE
fn warm_up<'a, F>(f: F) where F: Fn(&'a mut Bencher) {
}

fn main() {
    // ICE trigger
    warm_up(|b: &mut Bencher| () );

    // OK
    warm_up(|b| () );
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
