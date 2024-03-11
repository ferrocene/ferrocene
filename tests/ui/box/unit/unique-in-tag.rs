//@ run-pass
#![allow(dead_code)]
#![allow(non_camel_case_types)]

fn test1() {
    enum bar { u(Box<isize>), w(isize), }

    let x = bar::u(Box::new(10));
    assert!(match x {
      bar::u(a) => {
        println!("{}", a);
        *a
      }
      _ => { 66 }
    } == 10);
}

pub fn main() {
    test1();
}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_uloyjbaso8pz
// Path Patterns
//
// ferrocene-annotations: fls_d44aflefat88
// Path Pattern Matching
