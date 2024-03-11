//@ check-pass

#![allow(dropping_references)]

fn main() {}

struct U;

fn slice() {
    let mut arr = [U, U, U, U, U, U, U, U];
    let [ref _x0, _x1, _, mut _x3, .., ref _x6, _x7] = arr;
    _x3 = U;
    let [ref mut _x0, _, ref _x2, _, _x4, ref mut _x5, _x6, _] = arr;
    *_x5 = U;
    let [_, _, _x2, _, _, _x5, _, _] = arr;
    *_x0 = U;
    let [ref _x0, ..] = arr;
    let [_x0, ..] = arr;
}

fn tuple() {
    let mut tup = (U, U, U, U, U);
    let (ref _x0, mut _x1, ref _x2, ..) = tup;
    _x1 = U;
    let (ref mut _x0, _, _, ref _x3, _x4) = tup;
    let (_, _, _, _x3, _) = tup;
    *_x0 = U;
    drop(_x2);
    drop(tup.2);
    let (_x0, _, _, ..) = tup;
}

// ferrocene-annotations: fls_57ic33pwdvp3
// Slice Pattern Matching
//
// ferrocene-annotations: fls_qte70mgzpras
// Slice Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Underscore Pattern Matching
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Underscore Patterns
