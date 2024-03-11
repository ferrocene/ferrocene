//@ run-pass

use std::ops::Deref;

struct DerefArray<'a, T:'a> {
    inner: &'a [T]
}

impl<'a, T> Deref for DerefArray<'a, T> {
    type Target = &'a [T];

    fn deref<'b>(&'b self) -> &'b &'a [T] {
        &self.inner
    }
}

pub fn main() {
    let a = &[1, 2, 3];
    assert_eq!(DerefArray {inner: a}[1], 2);
}

// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
