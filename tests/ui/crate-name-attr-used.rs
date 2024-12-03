//@ run-pass
//@ compile-flags:--crate-name crate_name_attr_used -F unused-attributes


#![crate_name = "crate_name_attr_used"]

fn main() {}

// ferrocene-annotations: um_rustc_F
//
// ferrocene-annotations: fls_sun645voqex6
// Attribute crate_name
