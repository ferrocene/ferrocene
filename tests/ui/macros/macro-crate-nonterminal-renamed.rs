//@ run-pass
//@ aux-build:macro_crate_nonterminal.rs

#[macro_use]
extern crate macro_crate_nonterminal as new_name;

pub fn main() {
    new_name::check_local();
    assert_eq!(increment!(5), 6);
}

// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
//
// ferrocene-annotations: fls_gklst7joeo33
// External Crates
