//@ run-pass
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(non_camel_case_types)]


fn foo<T>(o: myoption<T>) -> isize {
    let mut x: isize;
    match o {
        myoption::none::<T> => { panic!(); }
        myoption::some::<T>(_t) => { x = 5; }
    }
    return x;
}

enum myoption<T> { none, some(T), }

pub fn main() { println!("{}", 5); }

// ferrocene-annotations: fls_d44aflefat88
// Path pattern matching
//
// ferrocene-annotations: fls_uloyjbaso8pz
// Path patterns
