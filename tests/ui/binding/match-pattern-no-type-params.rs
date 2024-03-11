//@ run-pass
#![allow(dead_code)]
#![allow(non_camel_case_types)]

enum maybe<T> { nothing, just(T), }

fn foo(x: maybe<isize>) {
    match x {
        maybe::nothing => { println!("A"); }
        maybe::just(_a) => { println!("B"); }
    }
}

pub fn main() { }

// ferrocene-annotations: fls_d44aflefat88
// Path pattern matching
//
// ferrocene-annotations: fls_uloyjbaso8pz
// Path patterns
