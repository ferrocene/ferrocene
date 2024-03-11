//@ check-pass

#[derive(
    core::clone::Clone,
    core::marker::Copy,
    core::fmt::Debug,
    core::default::Default,
    core::cmp::Eq,
    core::hash::Hash,
    core::cmp::Ord,
    core::cmp::PartialEq,
    core::cmp::PartialOrd,
)]
struct Core;

#[derive(
    std::clone::Clone,
    std::marker::Copy,
    std::fmt::Debug,
    std::default::Default,
    std::cmp::Eq,
    std::hash::Hash,
    std::cmp::Ord,
    std::cmp::PartialEq,
    std::cmp::PartialOrd,
)]
struct Std;

fn main() {
    core::column!();
    std::column!();
}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
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
