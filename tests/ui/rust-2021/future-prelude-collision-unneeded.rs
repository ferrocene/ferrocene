//@ edition:2018
//@ check-pass
#![allow(unused)]
#![deny(rust_2021_prelude_collisions)]

struct S;

impl S {
    fn try_into(self) -> S {
        S
    }
}

struct X;

trait Hey {
    fn from_iter(_: i32) -> Self;
}

impl Hey for X {
    fn from_iter(_: i32) -> Self {
        X
    }
}

struct Y<T>(T);

impl Hey for Y<i32> {
    fn from_iter(_: i32) -> Self {
        Y(0)
    }
}

struct Z<T>(T);

impl Hey for Z<i32> {
    fn from_iter(_: i32) -> Self {
        Z(0)
    }
}

impl std::iter::FromIterator<u32> for Z<u32> {
    fn from_iter<T: IntoIterator<Item = u32>>(_: T) -> Self {
        todo!()
    }
}

fn main() {
    // See https://github.com/rust-lang/rust/issues/86633
    let s = S;
    let s2 = s.try_into();

    // Check that we do not issue suggestions for types that do not implement `FromIter`.
    //
    // See https://github.com/rust-lang/rust/issues/86902
    X::from_iter(1);
    Y::from_iter(1);
    Y::<i32>::from_iter(1);
    Z::<i32>::from_iter(1);
}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
//
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
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
