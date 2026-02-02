//! Regression test for <https://github.com/rust-lang/rust/issues/151625>
#![expect(incomplete_features)]
#![feature(
    adt_const_params,
    min_generic_const_args,
    unsized_const_params
)]
fn foo<const X: (bool, i32)>() {}
fn bar<const Y: ([u8; 2], i32)>() {}
fn qux<const Z: (char, i32)>() {}

fn main() {
    foo::<{ (1, true) }>();
    //~^ ERROR: type annotations needed for the literal
    //~| ERROR: mismatched types: expected `i32`, found `bool`
    bar::<{ (1_u32, [1, 2]) }>();
    //~^ ERROR: expected `i32`, found const array
    //~| ERROR: mismatched types: expected `[u8; 2]`, found `u32`
    qux::<{ (1i32, 'a') }>();
    //~^ ERROR: mismatched types: expected `char`, found `i32`
    //~| ERROR: mismatched types: expected `i32`, found `char`
}
