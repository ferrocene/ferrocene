//@ run-pass
#![allow(non_camel_case_types)]

macro_rules! define_vec {
    () => (
        mod foo {
            #[derive(PartialEq)]
            pub struct bar;
        }
    )
}

define_vec![];

pub fn main() {}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
