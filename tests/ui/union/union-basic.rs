//@ run-pass

#![allow(unused_imports)]

//@ aux-build:union.rs

extern crate union;
use std::mem::{size_of, align_of, zeroed};

union U {
    a: u8,
    b: u16
}

fn local() {
    assert_eq!(size_of::<U>(), 2);
    assert_eq!(align_of::<U>(), 2);

    let u = U { a: 10 };
    unsafe {
        assert_eq!(u.a, 10);
        let U { a } = u;
        assert_eq!(a, 10);
    }

    let mut w = U { b: 0 };
    unsafe {
        assert_eq!(w.a, 0);
        assert_eq!(w.b, 0);
        w.a = 1;
        assert_eq!(w.a, 1);
        assert_eq!(w.b.to_le(), 1);
    }
}

fn xcrate() {
    assert_eq!(size_of::<union::U>(), 2);
    assert_eq!(align_of::<union::U>(), 2);

    let u = union::U { a: 10 };
    unsafe {
        assert_eq!(u.a, 10);
        let union::U { a } = u;
        assert_eq!(a, 10);
    }

    let mut w = union::U { b: 0 };
    unsafe {
        assert_eq!(w.a, 0);
        assert_eq!(w.b, 0);
        w.a = 1;
        assert_eq!(w.a, 1);
        assert_eq!(w.b.to_le(), 1);
    }
}

fn main() {
    local();
    xcrate();
}

// ferrocene-annotations: fls_8wnyln2nmg4y
// Unsafe Blocks
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
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
