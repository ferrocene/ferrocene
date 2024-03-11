//@ check-pass

#![allow(dead_code)]

trait Trait1<T, U> {
    fn f1(self) -> U;
}

trait Trait2 {
    type T;
    type U: Trait2<T = Self::T>;
    fn f2(f: impl FnOnce(&Self::U));
}

fn f3<T: Trait2>() -> impl Trait1<T, T::T> {
    Struct1
}

struct Struct1;

impl<T: Trait2> Trait1<T, T::T> for Struct1 {
    fn f1(self) -> T::T {
        unimplemented!()
    }
}

fn f4<T: Trait2>() {
    T::f2(|_| {
        f3::<T::U>().f1();
    });
}

fn main() {}

// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
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
//
// ferrocene-annotations: fls_1h0olpc7vbui
// Type Path Resolution
