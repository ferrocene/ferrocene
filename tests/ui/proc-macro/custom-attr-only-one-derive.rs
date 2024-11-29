//@ run-pass
//@ proc-macro: custom-attr-only-one-derive.rs

#[macro_use]
extern crate custom_attr_only_one_derive;

#[derive(Bar, Foo)]
#[custom = "test"]
pub enum A {
    B,
    C,
}

fn main() {}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
