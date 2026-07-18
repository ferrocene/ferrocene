extern crate std;
#[attr = PreludeImport]
use std::prelude::rust_2021::*;
//@ pretty-compare-only
//@ pretty-mode:hir
//@ pp-exact:hir-pretty-loop.pp

fn foo() { loop { break; } }
