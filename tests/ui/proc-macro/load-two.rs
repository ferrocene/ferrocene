//@ run-pass

#![allow(path_statements)]
#![allow(dead_code)]
//@ aux-build:derive-atob.rs
//@ aux-build:derive-ctod.rs

#[macro_use]
extern crate derive_atob;
#[macro_use]
extern crate derive_ctod;

#[derive(Copy, Clone)]
#[derive(AToB)]
struct A;

#[derive(CToD)]
struct C;

fn main() {
    B;
    D;
}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
//
// ferrocene-annotations: fls_q6qecp6e413
// Attribute proc_macro_derive
