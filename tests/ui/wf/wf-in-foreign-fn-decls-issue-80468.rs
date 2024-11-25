// Regression test for #80468.

#![crate_type = "lib"]

pub trait Trait {}

#[repr(transparent)]
pub struct Wrapper<T: Trait>(T);

#[repr(transparent)]
pub struct Ref<'a>(&'a u8);

impl Trait for Ref {} //~ ERROR:  implicit elided lifetime not allowed here

extern "C" {
    pub fn repro(_: Wrapper<Ref>);
}

// ferrocene-annotations: fls_ujig607lmwbm
// Attribute crate_type
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_aibb2quva4mn
// Attribute repr
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_rjxpof29a3nl
// Struct Type Representation
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_l9ebxrlxyawd
// Lifetime Elision
//
// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
//
// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
