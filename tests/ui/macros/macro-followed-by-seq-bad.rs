// Regression test for issue #25436: check that things which can be
// followed by any token also permit X* to come afterwards.

#![allow(unused_macros)]

macro_rules! foo {
  ( $a:expr $($b:tt)* ) => { }; //~ ERROR not allowed for `expr` fragments
  ( $a:ty $($b:tt)* ) => { };   //~ ERROR not allowed for `ty` fragments
}

fn main() { }

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
//
// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
//
// ferrocene-annotations: fls_k01lsksqtq1r
// Repetition
