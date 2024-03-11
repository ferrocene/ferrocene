//@ run-rustfix

pub fn foo(x: &mut Vec<&u8>, y: &u8) { x.push(y); } //~ ERROR lifetime may not live long enough

pub fn foo2(x: &mut Vec<&'_ u8>, y: &u8) { x.push(y); } //~ ERROR lifetime may not live long enough

pub fn foo3<'a>(_other: &'a [u8], x: &mut Vec<&u8>, y: &u8) { x.push(y); } //~ ERROR lifetime may not live long enough

fn main() {}

// ferrocene-annotations: fls_l9ebxrlxyawd
// Lifetime Elision
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Type
//
// ferrocene-annotations: fls_hethxxbcg7ja
// Function Lifetime Elision
