//@ aux-build:variant-namespacing.rs

pub enum E {
    Struct { a: u8 },
    Tuple(u8),
    Unit,
}

type Struct = u8;
type Tuple = u8;
type Unit = u8;
type XStruct = u8;
type XTuple = u8;
type XUnit = u8;

const Struct: u8 = 0;
const Tuple: u8 = 0;
const Unit: u8 = 0;
const XStruct: u8 = 0;
const XTuple: u8 = 0;
const XUnit: u8 = 0;

extern crate variant_namespacing;
pub use variant_namespacing::XE::{XStruct, XTuple, XUnit};
//~^ ERROR the name `XStruct` is defined multiple times
//~| ERROR the name `XTuple` is defined multiple times
//~| ERROR the name `XUnit` is defined multiple times
pub use E::{Struct, Tuple, Unit};
//~^ ERROR the name `Struct` is defined multiple times
//~| ERROR the name `Tuple` is defined multiple times
//~| ERROR the name `Unit` is defined multiple times

fn main() {}

// ferrocene-annotations: fls_kgvleup5mdhq
// Type Aliases
//
// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_gklst7joeo33
// External Crates
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
// ferrocene-annotations: fls_bbso3c45kr9z
// Simple Path Resolution
