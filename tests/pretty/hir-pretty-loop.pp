#[attr = MacroUse {arguments: UseAll}]
extern crate std;
#[prelude_import]
use std::prelude::rust_2021::*;
//@ pretty-compare-only
//@ pretty-mode:hir
//@ pp-exact:hir-pretty-loop.pp

fn foo() { loop { break; } }
