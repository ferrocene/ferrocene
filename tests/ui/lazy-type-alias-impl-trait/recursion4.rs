#![feature(type_alias_impl_trait)]

type Foo = impl std::fmt::Debug;

#[define_opaque(Foo)]
fn foo(b: bool) -> Foo {
    if b {
        return vec![];
    }
    let mut x = foo(false);
    x = std::iter::empty().collect(); //~ ERROR `Foo` cannot be built from an iterator
    vec![]
}

fn bar(b: bool) -> impl std::fmt::Debug {
    if b {
        return vec![];
    }
    let mut x = bar(false);
    x = std::iter::empty().collect(); //~ ERROR `impl Debug` cannot be built from an iterator
    vec![]
}

fn main() {}

// ferrocene-annotations: fls_3xqobbu7wfsf
// Impl Trait Type
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_kgvleup5mdhq
// Type Aliasing
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
