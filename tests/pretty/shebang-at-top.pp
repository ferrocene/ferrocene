#!/usr/bin/env rust
#![feature(prelude_import)]
#[macro_use]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
//@ pretty-mode:expanded
//@ pp-exact:shebang-at-top.pp
//@ pretty-compare-only

fn main() {}
