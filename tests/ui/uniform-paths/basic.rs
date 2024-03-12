//@ run-pass
//@ edition:2018

#![allow(unused_imports)]
#![allow(non_camel_case_types)]

// Test that ambiguity errors are not emitted between `self::test` and
// `::test`, assuming the latter (crate) is not in `extern_prelude`.
mod test {
    pub struct Foo(pub ());
}
use test::Foo;

// Test that qualified paths can refer to both the external crate and local item.
mod std {
    pub struct io(pub ());
}
use ::std::io as std_io;
use self::std::io as local_io;

fn main() {
    Foo(());
    let _ = std_io::stdout();
    local_io(());

    {
        // Test that having `std_io` in a module scope and a non-module
        // scope is allowed, when both resolve to the same definition.
        use ::std::io as std_io;
        use std_io::stdout;
        let _ = stdout();
    }
}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
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
// ferrocene-annotations: fls_o9u2h5m17kpz
// Path Expression Resolution
