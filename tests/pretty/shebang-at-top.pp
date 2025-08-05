#!/usr/bin/env rust
#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
//@ pretty-mode:expanded
//@ pp-exact:shebang-at-top.pp
//@ pretty-compare-only

fn main() {}
