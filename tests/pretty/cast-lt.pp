#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
//@ pretty-compare-only
//@ pretty-mode:expanded
//@ pp-exact:cast-lt.pp

macro_rules! negative { ($e:expr) => { $e < 0 } }

fn main() { (1 as i32) < 0; }
