#![feature(c_variadic)]
#![allow(anonymous_parameters)]

fn main() {}

fn f1_1(x: isize, ...) {}
//~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic

fn f1_2(...) {}
//~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic

extern "C" fn f2_1(x: isize, ...) {}
//~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic

extern "C" fn f2_2(...) {}
//~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic

extern "C" fn f2_3(..., x: isize) {}
//~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic
//~| ERROR `...` must be the last argument of a C-variadic function

extern "C" fn f3_1(x: isize, ...) {}
//~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic

extern "C" fn f3_2(...) {}
//~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic

extern "C" fn f3_3(..., x: isize) {}
//~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic
//~| ERROR `...` must be the last argument of a C-variadic function

const unsafe extern "C" fn f4_1(x: isize, ...) {}
//~^ ERROR functions cannot be both `const` and C-variadic
//~| ERROR destructor of `VaListImpl<'_>` cannot be evaluated at compile-time

const extern "C" fn f4_2(x: isize, ...) {}
//~^ ERROR functions cannot be both `const` and C-variadic
//~| ERROR only foreign or `unsafe extern "C"` functions may be C-variadic
//~| ERROR destructor of `VaListImpl<'_>` cannot be evaluated at compile-time

const extern "C" fn f4_3(..., x: isize, ...) {}
//~^ ERROR functions cannot be both `const` and C-variadic
//~| ERROR only foreign or `unsafe extern "C"` functions may be C-variadic
//~| ERROR `...` must be the last argument of a C-variadic function

extern "C" {
    fn e_f2(..., x: isize);
    //~^ ERROR `...` must be the last argument of a C-variadic function
}

struct X;

impl X {
    fn i_f1(x: isize, ...) {}
    //~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic
    fn i_f2(...) {}
    //~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic
    fn i_f3(..., x: isize, ...) {}
    //~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic
    //~| ERROR `...` must be the last argument of a C-variadic function
    fn i_f4(..., x: isize, ...) {}
    //~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic
    //~| ERROR `...` must be the last argument of a C-variadic function
    const fn i_f5(x: isize, ...) {}
    //~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic
    //~| ERROR functions cannot be both `const` and C-variadic
    //~| ERROR destructor of `VaListImpl<'_>` cannot be evaluated at compile-time
}

trait T {
    fn t_f1(x: isize, ...) {}
    //~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic
    fn t_f2(x: isize, ...);
    //~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic
    fn t_f3(...) {}
    //~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic
    fn t_f4(...);
    //~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic
    fn t_f5(..., x: isize) {}
    //~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic
    //~| ERROR `...` must be the last argument of a C-variadic function
    fn t_f6(..., x: isize);
    //~^ ERROR only foreign or `unsafe extern "C"` functions may be C-variadic
    //~| ERROR `...` must be the last argument of a C-variadic function
}

// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
