//@ aux-build:xcrate.rs
//@ compile-flags:--extern xcrate
//@ edition:2018

use crate; //~ ERROR crate root imports need to be explicitly named: `use crate as name;`
use *; //~ ERROR cannot glob-import all possible crates

fn main() {
    let s = ::xcrate; //~ ERROR expected value, found crate `xcrate`
                      //~^ NOTE not a value
}

// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
// ferrocene-annotations: fls_6l60b5hwnjbm
// Path Expressions
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
