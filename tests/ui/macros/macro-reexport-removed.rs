//@ aux-build:two_macros.rs
<<<<<<< HEAD
//@ normalize-stderr: "\(you are using [0-9]\.[0-9]+\.[0-9]+(.+)\)" -> "(you are using $$RUSTC_VERSION)"
=======
//@ normalize-stderr: "you are using [0-9]+\.[0-9]+\.[0-9]+(-[a-zA-Z0-9.]+)?( \([^)]*\))?" -> "you are using $$RUSTC_VERSION"
>>>>>>> pull-upstream-temp--do-not-use-for-real-code

#![feature(macro_reexport)] //~ ERROR feature has been removed

#[macro_reexport(macro_one)] //~ ERROR cannot find attribute `macro_reexport` in this scope
extern crate two_macros;

fn main() {}
