//@ aux-build:define-macro.rs

macro_rules! bar { () => {} }
define_macro!(bar);
bar!(); //~ ERROR `bar` is ambiguous

macro_rules! m { () => { #[macro_use] extern crate define_macro; } }
m!();

fn main() {}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
