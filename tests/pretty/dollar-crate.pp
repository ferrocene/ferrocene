#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
//@ pretty-compare-only
//@ pretty-mode:expanded
//@ pp-exact:dollar-crate.pp

fn main() { { ::std::io::_print(format_args!("rust\n")); }; }
