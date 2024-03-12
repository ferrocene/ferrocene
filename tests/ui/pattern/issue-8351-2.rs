//@ run-pass
#![allow(dead_code)]

enum E {
    Foo{f: isize, b: bool},
    Bar,
}

pub fn main() {
    let e = E::Foo{f: 0, b: false};
    match e {
        E::Foo{f: 1, b: true} => panic!(),
        E::Foo{b: false, f: 0} => (),
        _ => panic!(),
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
