//@ run-pass
#![allow(dead_code)]
#![allow(unused_variables)]

struct A<'a, 'b> where 'a : 'b { x: &'a isize, y: &'b isize }

fn main() {
    let x = 1;
    let y = 1;
    let a = A { x: &x, y: &y };
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
