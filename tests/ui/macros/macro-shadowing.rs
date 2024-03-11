//@ aux-build:two_macros.rs

#![allow(unused_macros)]

macro_rules! foo { () => {} }
macro_rules! macro_one { () => {} }
#[macro_use(macro_two)] extern crate two_macros;

macro_rules! m1 { () => {
    macro_rules! foo { () => {} }

    #[macro_use] //~ ERROR `macro_two` is already in scope
    extern crate two_macros as __;
}}
m1!();

foo!(); //~ ERROR `foo` is ambiguous

macro_rules! m2 { () => {
    macro_rules! foo { () => {} }
    foo!();
}}
m2!();
//^ Since `foo` is not used outside this expansion, it is not a shadowing error.

fn main() {}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
//
// ferrocene-annotations: fls_ym00b6ewf4n3
// Macro Transcription
//
// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
//
// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
