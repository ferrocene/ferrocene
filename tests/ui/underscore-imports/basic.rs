//@ check-pass
//@ aux-build:underscore-imports.rs

#![warn(unused_imports, unused_extern_crates)]

#[macro_use]
extern crate underscore_imports as _;

do_nothing!(); // OK

struct S;

mod m {
    pub trait Tr1 {
        fn tr1_is_in_scope(&self) {}
    }
    pub trait Tr2 {
        fn tr2_is_in_scope(&self) {}
    }

    impl Tr1 for ::S {}
    impl Tr2 for ::S {}
}

mod unused {
    use m::Tr1 as _; //~ WARN unused import
    use S as _; //~ WARN unused import
    extern crate core as _; // OK
}

mod outer {
    mod middle {
        pub use m::Tr1 as _;
        pub use m::Tr2 as _; // OK, no name conflict
        struct Tr1; // OK, no name conflict
        fn check() {
            // Both traits are in scope
            ::S.tr1_is_in_scope();
            ::S.tr2_is_in_scope();
        }

        mod inner {
            // `_` imports are fetched by glob imports
            use super::*;
            fn check() {
                // Both traits are in scope
                ::S.tr1_is_in_scope();
                ::S.tr2_is_in_scope();
            }
        }
    }

    // `_` imports are fetched by glob imports
    use self::middle::*;
    fn check() {
        // Both traits are in scope
        ::S.tr1_is_in_scope();
        ::S.tr2_is_in_scope();
    }
}

fn main() {}

// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
//
// ferrocene-annotations: fls_gklst7joeo33
// External Crates
//
// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
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
// ferrocene-annotations: fls_bbso3c45kr9z
// Simple Path Resolution
//
// ferrocene-annotations: fls_1h0olpc7vbui
// Type Path Resolution
