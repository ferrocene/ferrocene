//@ run-pass
#![allow(unused_variables)]
#![allow(non_upper_case_globals)]

// Empty struct defined with braces add names into type namespace
// Empty struct defined without braces add names into both type and value namespaces

//@ aux-build:empty-struct.rs

extern crate empty_struct;
use empty_struct::*;

struct Empty1 {}
struct Empty2;
struct Empty7();

#[derive(PartialEq, Eq)]
struct Empty3 {}

const Empty3: Empty3 = Empty3 {};

enum E {
    Empty4 {},
    Empty5,
    Empty6(),
}

fn local() {
    let e1: Empty1 = Empty1 {};
    let e2: Empty2 = Empty2 {};
    let e2: Empty2 = Empty2;
    let e3: Empty3 = Empty3 {};
    let e3: Empty3 = Empty3;
    let e4: E = E::Empty4 {};
    let e5: E = E::Empty5 {};
    let e5: E = E::Empty5;
    let e6: E = E::Empty6 {};
    let e6: E = E::Empty6();
    let ctor6: fn() -> E = E::Empty6;
    let e7: Empty7 = Empty7 {};
    let e7: Empty7 = Empty7();
    let ctor7: fn() -> Empty7 = Empty7;

    match e1 {
        Empty1 {} => {}
    }
    match e2 {
        Empty2 {} => {}
    }
    match e3 {
        Empty3 {} => {}
    }
    match e4 {
        E::Empty4 {} => {}
        _ => {}
    }
    match e5 {
        E::Empty5 {} => {}
        _ => {}
    }
    match e6 {
        E::Empty6 {} => {}
        _ => {}
    }
    match e7 {
        Empty7 {} => {}
    }

    match e1 {
        Empty1 { .. } => {}
    }
    match e2 {
        Empty2 { .. } => {}
    }
    match e3 {
        Empty3 { .. } => {}
    }
    match e4 {
        E::Empty4 { .. } => {}
        _ => {}
    }
    match e5 {
        E::Empty5 { .. } => {}
        _ => {}
    }
    match e6 {
        E::Empty6 { .. } => {}
        _ => {}
    }
    match e7 {
        Empty7 { .. } => {}
    }

    match e2 {
        Empty2 => {}
    }
    match e3 {
        Empty3 => {}
    }
    match e5 {
        E::Empty5 => {}
        _ => {}
    }
    match e6 {
        E::Empty6() => {}
        _ => {}
    }
    match e6 {
        E::Empty6(..) => {}
        _ => {}
    }
    match e7 {
        Empty7() => {}
    }
    match e7 {
        Empty7(..) => {}
    }

    let e11: Empty1 = Empty1 { ..e1 };
    let e22: Empty2 = Empty2 { ..e2 };
    let e33: Empty3 = Empty3 { ..e3 };
    let e77: Empty7 = Empty7 { ..e7 };
}

fn xcrate() {
    let e1: XEmpty1 = XEmpty1 {};
    let e2: XEmpty2 = XEmpty2 {};
    let e2: XEmpty2 = XEmpty2;
    let e3: XE = XE::XEmpty3 {};
    let e4: XE = XE::XEmpty4 {};
    let e4: XE = XE::XEmpty4;
    let e6: XE = XE::XEmpty6 {};
    let e6: XE = XE::XEmpty6();
    let ctor6: fn() -> XE = XE::XEmpty6;
    let e7: XEmpty7 = XEmpty7 {};
    let e7: XEmpty7 = XEmpty7();
    let ctor7: fn() -> XEmpty7 = XEmpty7;

    match e1 {
        XEmpty1 {} => {}
    }
    match e2 {
        XEmpty2 {} => {}
    }
    match e3 {
        XE::XEmpty3 {} => {}
        _ => {}
    }
    match e4 {
        XE::XEmpty4 {} => {}
        _ => {}
    }
    match e6 {
        XE::XEmpty6 {} => {}
        _ => {}
    }
    match e7 {
        XEmpty7 {} => {}
    }

    match e1 {
        XEmpty1 { .. } => {}
    }
    match e2 {
        XEmpty2 { .. } => {}
    }
    match e3 {
        XE::XEmpty3 { .. } => {}
        _ => {}
    }
    match e4 {
        XE::XEmpty4 { .. } => {}
        _ => {}
    }
    match e6 {
        XE::XEmpty6 { .. } => {}
        _ => {}
    }
    match e7 {
        XEmpty7 { .. } => {}
    }

    match e2 {
        XEmpty2 => {}
    }
    match e4 {
        XE::XEmpty4 => {}
        _ => {}
    }
    match e6 {
        XE::XEmpty6() => {}
        _ => {}
    }
    match e6 {
        XE::XEmpty6(..) => {}
        _ => {}
    }
    match e7 {
        XEmpty7() => {}
    }
    match e7 {
        XEmpty7(..) => {}
    }

    let e11: XEmpty1 = XEmpty1 { ..e1 };
    let e22: XEmpty2 = XEmpty2 { ..e2 };
    let e77: XEmpty7 = XEmpty7 { ..e7 };
}

fn main() {
    local();
    xcrate();
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_dq403wq5yrs
// Namespaces
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
