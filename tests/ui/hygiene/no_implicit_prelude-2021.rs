//@ check-pass
//@ edition:2021

#![no_implicit_prelude]

fn main() {
    assert!(true, "hoi");
    assert!(false, "hoi {}", 123);
}

// ferrocene-annotations: fls_iikmhqsp1r5a
// Attribute no_implicit_prelude
