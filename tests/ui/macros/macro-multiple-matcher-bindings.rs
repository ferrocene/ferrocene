// Test that duplicate matcher binding names are caught at declaration time, rather than at macro
// invocation time.

#![allow(unused_macros)]

macro_rules! foo1 {
    ($a:ident, $a:ident) => {}; //~ERROR duplicate matcher binding
    ($a:ident, $a:path) => {};  //~ERROR duplicate matcher binding
}

macro_rules! foo2 {
    ($a:ident) => {}; // OK
    ($a:path) => {};  // OK
}

macro_rules! foo3 {
    ($a:ident, $($a:ident),*) => {}; //~ERROR duplicate matcher binding
    ($($a:ident)+ # $($($a:path),+);*) => {}; //~ERROR duplicate matcher binding
}

fn main() {}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
//
// ferrocene-annotations: fls_k01lsksqtq1r
// Repetition
