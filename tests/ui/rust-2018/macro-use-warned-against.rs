//@ aux-build:macro-use-warned-against.rs
//@ aux-build:macro-use-warned-against2.rs
//@ check-pass

#![warn(macro_use_extern_crate, unused)]

#[macro_use] //~ WARN should be replaced at use sites with a `use` item
extern crate macro_use_warned_against;
#[macro_use] //~ WARN unused `#[macro_use]`
extern crate macro_use_warned_against2;

fn main() {
    foo!();
}

// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
//
// ferrocene-annotations: fls_gklst7joeo33
// External Crates
