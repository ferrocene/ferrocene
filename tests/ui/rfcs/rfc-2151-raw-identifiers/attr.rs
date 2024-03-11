//@ run-pass
use std::mem;

#[r#repr(r#C, r#packed)]
struct Test {
    a: bool, b: u64
}

#[r#derive(r#Debug)]
struct Test2(#[allow(dead_code)] u32);

pub fn main() {
    assert_eq!(mem::size_of::<Test>(), 9);
    assert_eq!("Test2(123)", format!("{:?}", Test2(123)));
}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
//
// ferrocene-annotations: fls_aibb2quva4mn
// Attribute repr
