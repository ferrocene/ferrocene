//@ run-pass
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_assignments)]
#![allow(unused_variables)]
#![allow(stable_features)]
#![allow(dropping_copy_types)]

// Test parsing binary operators after macro invocations.


#![feature(macro_rules)]

macro_rules! id {
    ($e: expr) => { $e }
}

fn foo() {
    id!(1) + 1;
    id![1] - 1;
    id!(1) * 1;
    id![1] / 1;
    id!(1) % 1;

    id!(1) & 1;
    id![1] | 1;
    id!(1) ^ 1;

    let mut x = 1;
    id![x] = 2;
    id!(x) += 1;

    id!(1f64).clone();

    id!([1, 2, 3])[1];
    id![drop](1);

    id!(true) && true;
    id![true] || true;
}

fn main() {}

// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic Expressions
//
// ferrocene-annotations: fls_abp6tjbz8tpn
// Bit Expressions
//
// ferrocene-annotations: fls_nsvzzbldhq53
// Comparison Expressions
//
// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_lstusiu2c8lu
// Lazy Boolean Expressions
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
