#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
//@ pretty-compare-only
//@ pretty-mode:expanded
//@ pp-exact:dollar-crate.pp

fn main() { { ::std::io::_print(format_args!("rust\n")); }; }
