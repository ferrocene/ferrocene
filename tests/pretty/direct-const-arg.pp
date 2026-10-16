#![feature(prelude_import)]
//@ pretty-mode:expanded
//@ pp-exact:direct-const-arg.pp
#![feature(min_generic_const_args)]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;

fn f<const N : usize>() {}

fn main() {
    f::<core::direct_const_arg! (2)>();
    f::<{ core::direct_const_arg! (2) }>();
}
