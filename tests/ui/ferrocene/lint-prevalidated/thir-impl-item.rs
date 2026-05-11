// Make sure our THIR visitor recurses into functions in impl blocks.

//@ check-fail
#![crate_type = "lib"]
#![deny(ferrocene::unvalidated)]

pub struct A(u32);

const fn unvalidated() {}

impl A {
    #[ferrocene::prevalidated]
    #[inline]
    pub const fn b(&self) -> u32 {
        unvalidated(); //~ ERROR: unvalidated
        0
    }
}
