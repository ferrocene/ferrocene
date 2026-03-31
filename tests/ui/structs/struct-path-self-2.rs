//@ run-pass
use std::ops::Add;

struct S<T, U = u16> {
    a: T,
    b: U,
}

trait Tr {
    fn f(&self) -> Self;
}

impl<T: Default + Add<u8, Output = T>, U: Default> Tr for S<T, U> {
    fn f(&self) -> Self {
        let s = Self { a: Default::default(), b: Default::default() };
        match s {
            Self { a, b } => Self { a: a + 1, b: b }
        }
    }
}

impl<T: Default, U: Default + Add<u16, Output = U>> S<T, U> {
    fn g(&self) -> Self {
        let s = Self { a: Default::default(), b: Default::default() };
        match s {
            Self { a, b } => Self { a: a, b: b + 1 }
        }
    }
}

impl S<u8> {
    fn new() -> Self {
        Self { a: 0, b: 1 }
    }
}

fn main() {
    let s0 = S::new();
    let s1 = s0.f();
    assert_eq!(s1.a, 1);
    assert_eq!(s1.b, 0);
    let s2 = s0.g();
    assert_eq!(s2.a, 0);
    assert_eq!(s2.b, 1);
}

// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_jm6l7b90h6wa
// Pattern Matching
//
// ferrocene-annotations: fls_151r19d7xbgz
// Entities
//
// ferrocene-annotations: fls_izl8iuhoz9e0
// Scopes
//
// ferrocene-annotations: fls_6ozthochxz1i
// Binding Scopes
//
// ferrocene-annotations: fls_ftphlagzd2te
// Generic Parameter Scope
//
// ferrocene-annotations: fls_m0z7omni9hp0
// Item Scope
//
// ferrocene-annotations: fls_769b4p8v3cwu
// Label Scope
//
// ferrocene-annotations: fls_kgbi26212eof
// Self Scope
//
// ferrocene-annotations: fls_octf6sf7yso
// Textual Macro Scope
//
// ferrocene-annotations: fls_lnpyb285qdiy
// Scope Hierarchy
//
// ferrocene-annotations: fls_dq403wq5yrs
// Namespaces
//
// ferrocene-annotations: fls_ld0ize96cm6m
// Preludes
//
// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
//
// ferrocene-annotations: fls_40xoego2thsp
// Resolution
//
// ferrocene-annotations: fls_i6qzga6dyaee
// Path Resolution
//
// ferrocene-annotations: fls_o9u2h5m17kpz
// Path Expression Resolution
//
// ferrocene-annotations: fls_1h0olpc7vbui
// Type Path Resolution
