//@ aux-build:types.rs
#![deny(improper_ctypes)]

extern crate types;

// This test checks that non-exhaustive types with `#[repr(C)]` from an extern crate are considered
// improper.

use types::{
    NonExhaustiveCLikeEnum, NonExhaustiveEnum, NonExhaustiveVariants,
    NormalStruct, TupleStruct, UnitStruct,
};

extern "C" {
    pub fn non_exhaustive_enum(_: NonExhaustiveEnum);
    //~^ ERROR `extern` block uses type `NonExhaustiveEnum`, which is not FFI-safe
    pub fn non_exhaustive_normal_struct(_: NormalStruct);
    //~^ ERROR `extern` block uses type `NormalStruct`, which is not FFI-safe
    pub fn non_exhaustive_unit_struct(_: UnitStruct);
    //~^ ERROR `extern` block uses type `UnitStruct`, which is not FFI-safe
    pub fn non_exhaustive_tuple_struct(_: TupleStruct);
    //~^ ERROR `extern` block uses type `TupleStruct`, which is not FFI-safe
    pub fn non_exhaustive_variant(_: NonExhaustiveVariants);
    //~^ ERROR `extern` block uses type `NonExhaustiveVariants`, which is not FFI-safe
}

// These should pass without remark, as they're C-compatible, despite being "non-exhaustive".
extern "C" {
    pub fn non_exhaustive_c_compat_enum(_: NonExhaustiveCLikeEnum);
}

fn main() {}

// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
