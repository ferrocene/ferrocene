//@ run-pass

#![allow(unused_must_use)]
#![allow(path_statements)]
//@ proc-macro: derive-a.rs

#[macro_use]
extern crate derive_a;

#[derive(Debug, PartialEq, A, Eq, Copy, Clone)]
struct A;

fn main() {
    A;
    assert_eq!(A, A);
    A.clone();
    let a = A;
    let _c = a;
    let _d = a;
}

// ferrocene-annotations: fls_o8s3r7m90q59
// Derive Macros
//
// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
