#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
// Test for issue 80832
//
//@ pretty-mode:expanded
//@ pp-exact:expanded-and-path-remap-80832.pp
//@ compile-flags: --remap-path-prefix {{src-base}}=the/src

fn main() {}
