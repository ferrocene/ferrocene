#![feature(lint_reasons)]

#![deny(unused_attributes)]
#![allow()] //~ ERROR unused attribute
#![expect()] //~ ERROR unused attribute
#![warn()] //~ ERROR unused attribute
#![deny()] //~ ERROR unused attribute
#![forbid()] //~ ERROR unused attribute
#![feature()] //~ ERROR unused attribute

#[repr()] //~ ERROR unused attribute
pub struct S;

#[target_feature()] //~ ERROR unused attribute
pub unsafe fn foo() {}

fn main() {}

// ferrocene-annotations: fls_ahmnqhm8anlb
// Built-in Attributes
//
// ferrocene-annotations: fls_spdmit5fy7el
// Attribute target_feature
