#![feature(adt_const_params, min_generic_const_args)]
//~^ WARN feature `min_generic_const_args` is incomplete
use std::marker::ConstParamTy;

#[derive(Eq, PartialEq, ConstParamTy)]
struct Foo;

struct Bar;

fn test<const N: [Foo; 1]>() {}

fn main() {
    test::<{ [Bar] }>();
    //~^ ERROR constant `Bar` is not of type `Foo`
}
