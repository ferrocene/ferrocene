//@ run-pass
#![allow(dead_code)]
#![allow(non_camel_case_types)]

enum bar { u(Box<isize>), w(isize), }

pub fn main() {
    let v = match bar::u(10.into()) {
        bar::u(a) => {
            println!("{}", a);
            *a
        }
        _ => { 66 }
    };
    assert_eq!(v, 10);
}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_uloyjbaso8pz
// Path Patterns
//
// ferrocene-annotations: fls_d44aflefat88
// Path Pattern Matching
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
