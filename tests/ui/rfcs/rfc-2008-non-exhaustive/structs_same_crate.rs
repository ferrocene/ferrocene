//@ run-pass

#![allow(unused_variables)]

#[non_exhaustive]
pub struct NormalStruct {
    pub first_field: u16,
    pub second_field: u16,
}

#[non_exhaustive]
pub struct UnitStruct;

#[non_exhaustive]
pub struct TupleStruct (pub u16, pub u16);

fn main() {
    let ns = NormalStruct { first_field: 640, second_field: 480 };

    let NormalStruct { first_field, second_field } = ns;

    let ts = TupleStruct { 0: 340, 1: 480 };
    let ts_constructor = TupleStruct(340, 480);

    let TupleStruct { 0: first, 1: second } = ts;
    let TupleStruct(first, second) = ts_constructor;

    let us = UnitStruct {};
    let us_constructor = UnitStruct;

    let UnitStruct { } = us;
}

// ferrocene-annotations: fls_asj8rgccvkoe
// Struct Pattern Matching
// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
// ferrocene-annotations: fls_yivm43r5wnp1
// Let Statements
// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
// ferrocene-annotations: fls_eexupzdsu7f
// Tuple Struct Pattern Matching
