//@ edition:2018
//@ proc-macro: builtin-attrs.rs
#![feature(decl_macro)] //~ ERROR `feature` is ambiguous

extern crate builtin_attrs;
use builtin_attrs::*;
use builtin_attrs::{bench, test};

#[repr(C)] //~ ERROR `repr` is ambiguous
struct S;
#[cfg_attr(all(), repr(C))] //~ ERROR `repr` is ambiguous
struct SCond;

#[test] // OK, shadowed
fn test() {}

#[bench] // OK, shadowed
fn bench() {}

fn non_macro_expanded_location<#[repr(C)] T>() {
    //~^ ERROR `repr` is ambiguous
    //~| ERROR attribute should be applied to a struct, enum, or union
    match 0u8 {
        #[repr(C)]
        //~^ ERROR `repr` is ambiguous
        //~| ERROR attribute should be applied to a struct, enum, or union
        _ => {}
    }
}

fn main() {
    Test;
    Bench;
    NonExistent; //~ ERROR cannot find value `NonExistent` in this scope
}

use deny as allow;
#[allow(unused)] //~ ERROR `allow` is ambiguous
fn builtin_renamed() {}

// ferrocene-annotations: fls_dv2j1fvvnk1t
// Attribute test
//
// ferrocene-annotations: fls_dd9xh3wdjudo
// Attribute cfg_attr
//
// ferrocene-annotations: fls_aibb2quva4mn
// Attribute repr
