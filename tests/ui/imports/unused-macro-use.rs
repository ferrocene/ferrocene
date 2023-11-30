#![deny(unused)]

#[macro_use] //~ ERROR unused `#[macro_use]` import
extern crate core;

#[macro_use(
    panic //~ ERROR unused `#[macro_use]` import
)]
extern crate core as core_2;

fn main() {}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_gklst7joeo33
// External Crates
