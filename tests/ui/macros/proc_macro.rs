//@ run-pass
//@ proc-macro: proc_macro_def.rs

extern crate proc_macro_def;

use proc_macro_def::{attr_tru, attr_identity, identity, ret_tru, tru};

#[attr_tru]
fn f1() -> bool {
    return false;
}

#[attr_identity]
fn f2() -> bool {
    return identity!(true);
}

fn f3() -> identity!(bool) {
    ret_tru!();
}

fn f4(x: bool) -> bool {
    match x {
        identity!(true) => false,
        identity!(false) => true,
    }
}

fn main() {
    assert!(f1());
    assert!(f2());
    assert!(tru!());
    assert!(f3());
    assert!(identity!(5 == 5));
    assert!(f4(false));
}

// ferrocene-annotations: fls_4vjbkm4ceymk
// Attribute Macros
//
// ferrocene-annotations: fls_qkmkev85o5jf
// Attribute proc_macro
//
// ferrocene-annotations: fls_ejhlylrcajo
// Attribute proc_macro_attribute
//
// ferrocene-annotations: fls_2d6bqnpy6tvs
// Function-like Macros
//
// ferrocene-annotations: fls_wn1i6hzg2ff7
// Procedural Macros
