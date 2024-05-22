//@ aux-build:static_priv_by_default.rs

extern crate static_priv_by_default;

fn foo<T>() {}

fn main() {
    // Actual public items should be public
    static_priv_by_default::a;
    static_priv_by_default::b;
    static_priv_by_default::c;
    foo::<static_priv_by_default::d>();
    foo::<static_priv_by_default::e>();

    // publicly re-exported items should be available
    static_priv_by_default::bar::e;
    static_priv_by_default::bar::f;
    static_priv_by_default::bar::g;
    foo::<static_priv_by_default::bar::h>();
    foo::<static_priv_by_default::bar::i>();

    // private items at the top should be inaccessible
    static_priv_by_default::j;
    //~^ ERROR: static `j` is private
    static_priv_by_default::k;
    //~^ ERROR: function `k` is private
    static_priv_by_default::l;
    //~^ ERROR: struct `l` is private
    foo::<static_priv_by_default::m>();
    //~^ ERROR: enum `m` is private
    foo::<static_priv_by_default::n>();
    //~^ ERROR: type alias `n` is private

    // public items in a private mod should be inaccessible
    static_priv_by_default::foo::a;
    //~^ ERROR: module `foo` is private
    static_priv_by_default::foo::b;
    //~^ ERROR: module `foo` is private
    static_priv_by_default::foo::c;
    //~^ ERROR: module `foo` is private
    foo::<static_priv_by_default::foo::d>();
    //~^ ERROR: module `foo` is private
    foo::<static_priv_by_default::foo::e>();
    //~^ ERROR: module `foo` is private
}

// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
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
