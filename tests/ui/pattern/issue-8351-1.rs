//@ run-pass
#![allow(dead_code)]

enum E {
    Foo{f: isize},
    Bar,
}

pub fn main() {
    let e = E::Foo{f: 0};
    match e {
        E::Foo{f: 1} => panic!(),
        E::Foo{..} => (),
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
