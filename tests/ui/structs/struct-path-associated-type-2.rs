//@ run-pass
#![allow(dead_code)]
struct S<T, U = u16> {
    a: T,
    b: U,
}

trait Tr {
    type A;
}
impl Tr for u8 {
    type A = S<u8, u16>;
}

fn f<T: Tr<A = S<u8>>>() {
    let s = T::A { a: 0, b: 1 };
    match s {
        T::A { a, b } => {
            assert_eq!(a, 0);
            assert_eq!(b, 1);
        }
    }
}

fn main() {
    f::<u8>();
}

// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
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
