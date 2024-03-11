//@ run-pass
#![allow(dead_code)]
#![allow(non_camel_case_types)]

enum opt<T> { none, some(T) }

pub fn main() {
    let x = opt::none::<isize>;
    match x {
        opt::none::<isize> => { println!("hello world"); }
        opt::some(_) => { }
    }
}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
