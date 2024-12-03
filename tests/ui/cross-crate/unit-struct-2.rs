//@ run-pass
//@ aux-build:xcrate_unit_struct.rs
#![allow(non_upper_case_globals)]

extern crate xcrate_unit_struct;

const s1: xcrate_unit_struct::Struct = xcrate_unit_struct::Struct;
static s2: xcrate_unit_struct::Unit = xcrate_unit_struct::Unit::UnitVariant;
static s3: xcrate_unit_struct::Unit =
                xcrate_unit_struct::Unit::Argument(xcrate_unit_struct::Struct);
static s4: xcrate_unit_struct::Unit = xcrate_unit_struct::Unit::Argument(s1);
static s5: xcrate_unit_struct::TupleStruct = xcrate_unit_struct::TupleStruct(20, "foo");

fn f1(_: xcrate_unit_struct::Struct) {}
fn f2(_: xcrate_unit_struct::Unit) {}
fn f3(_: xcrate_unit_struct::TupleStruct) {}

pub fn main() {
    f1(xcrate_unit_struct::Struct);
    f2(xcrate_unit_struct::Unit::UnitVariant);
    f2(xcrate_unit_struct::Unit::Argument(xcrate_unit_struct::Struct));
    f3(xcrate_unit_struct::TupleStruct(10, "bar"));

    f1(s1);
    f2(s2);
    f2(s3);
    f2(s4);
    f3(s5);
}

// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_xdvdl2ssnhlo
// Statics
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
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
// ferrocene-annotations: fls_o9u2h5m17kpz
// Path Expression Resolution
//
// ferrocene-annotations: fls_1h0olpc7vbui
// Type Path Resolution
