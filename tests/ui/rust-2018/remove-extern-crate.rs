//@ run-rustfix
//@ edition:2018
//@ check-pass
//@ aux-build:remove-extern-crate.rs
//@ compile-flags:--extern remove_extern_crate

#![warn(rust_2018_idioms)]
#![allow(dropping_copy_types)]
#![allow(unused_imports)]

extern crate core; //~ WARNING unused extern crate
// Shouldn't suggest changing to `use`, as `another_name`
// would no longer be added to the prelude which could cause
// compilation errors for imports that use `another_name` in other
// modules. See #57672.
extern crate core as another_name;
use remove_extern_crate;
#[macro_use]
extern crate remove_extern_crate as something_else;

// Shouldn't suggest changing to `use`, as the `alloc`
// crate is not in the extern prelude - see #54381.
extern crate alloc;

fn main() {
    another_name::mem::drop(3);
    another::foo();
    with_visibility::foo();
    remove_extern_crate::foo!();
    bar!();
    alloc::vec![5];
}

mod another {
    extern crate core; //~ WARNING `extern crate` is not idiomatic
    use remove_extern_crate;

    pub fn foo() {
        core::mem::drop(4);
        remove_extern_crate::foo!();
    }
}

mod with_visibility {
    pub extern crate core; //~ WARNING `extern crate` is not idiomatic

    pub fn foo() {
        core::mem::drop(4);
        remove_extern_crate::foo!();
    }
}

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
//
// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
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
// ferrocene-annotations: fls_bbso3c45kr9z
// Simple Path Resolution
//
// ferrocene-annotations: fls_o9u2h5m17kpz
// Path Expression Resolution
