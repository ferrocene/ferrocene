//@ run-pass

#![allow(dead_code)]
//@ proc-macro: derive-nothing.rs

#[macro_use]
extern crate derive_nothing;

macro_rules! int {
    () => { i32 }
}

#[derive(Nothing)]
struct S {
    x: int!(),
}

fn main() {}

// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
//
// ferrocene-annotations: fls_o8s3r7m90q59
// Derive Macros
