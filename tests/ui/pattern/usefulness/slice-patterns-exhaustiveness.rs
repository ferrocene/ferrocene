fn main() {
    let s: &[bool] = &[true; 0];
    let s1: &[bool; 1] = &[false; 1];
    let s2: &[bool; 2] = &[false; 2];
    let s3: &[bool; 3] = &[false; 3];
    let s10: &[bool; 10] = &[false; 10];

    match s2 {
        //~^ ERROR `&[false, _]` not covered
        [true, .., true] => {}
    }
    match s3 {
        //~^ ERROR `&[false, ..]` not covered
        [true, .., true] => {}
    }
    match s10 {
        //~^ ERROR `&[false, ..]` not covered
        [true, .., true] => {}
    }

    match s1 {
        [true, ..] => {}
        [.., false] => {}
    }
    match s2 {
        //~^ ERROR `&[false, true]` not covered
        [true, ..] => {}
        [.., false] => {}
    }
    match s3 {
        //~^ ERROR `&[false, .., true]` not covered
        [true, ..] => {}
        [.., false] => {}
    }
    match s {
        //~^ ERROR `&[false, .., true]` not covered
        [] => {}
        [true, ..] => {}
        [.., false] => {}
    }

    match s {
        //~^ ERROR `&[_, ..]` not covered
        [] => {}
    }
    match s {
        //~^ ERROR `&[_, _, ..]` not covered
        [] => {}
        [_] => {}
    }
    match s {
        //~^ ERROR `&[false, ..]` not covered
        [] => {}
        [true, ..] => {}
    }
    match s {
        //~^ ERROR `&[false, _, ..]` not covered
        [] => {}
        [_] => {}
        [true, ..] => {}
    }
    match s {
        //~^ ERROR `&[_, .., false]` not covered
        [] => {}
        [_] => {}
        [.., true] => {}
    }

    match s {
        //~^ ERROR `&[_, _, .., true]` not covered
        [] => {}
        [_] => {}
        [_, _] => {}
        [.., false] => {}
    }
    match s {
        //~^ ERROR `&[true, _, .., _]` not covered
        [] => {}
        [_] => {}
        [_, _] => {}
        [false, .., false] => {}
    }

    const CONST: &[bool] = &[true];
    match s {
        //~^ ERROR `&[]` and `&[_, _, ..]` not covered
        &[true] => {}
    }
    match s {
        //~^ ERROR `&[]` and `&[_, _, ..]` not covered
        CONST => {}
    }
    match s {
        //~^ ERROR `&[]` and `&[_, _, ..]` not covered
        CONST => {}
        &[false] => {}
    }
    match s {
        //~^ ERROR `&[]` and `&[_, _, ..]` not covered
        &[false] => {}
        CONST => {}
    }
    match s {
        //~^ ERROR `&[_, _, ..]` not covered
        &[] => {}
        CONST => {}
    }
    match s {
        //~^ ERROR `&[false]` not covered
        &[] => {}
        CONST => {}
        &[_, _, ..] => {}
    }
    match s {
        [] => {}
        [false] => {}
        CONST => {}
        [_, _, ..] => {}
    }
    const CONST1: &[bool; 1] = &[true];
    match s1 {
        //~^ ERROR `&[false]` not covered
        CONST1 => {}
    }
    match s1 {
        CONST1 => {}
        [false] => {}
    }
}

// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_vpbikb73dw4k
// Slice Types
//
// ferrocene-annotations: fls_xinykul167l
// Array Expressions
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_qte70mgzpras
// Slice Patterns
//
// ferrocene-annotations: fls_7wpgnp4kjq82
// Rest Patterns
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Underscore Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Underscore Pattern Matching
//
// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_2krxnq8q9ef1
// Literal Patterns
//
// ferrocene-annotations: fls_azzf1llv3wf
// Literal Pattern Matching
//
// ferrocene-annotations: fls_57ic33pwdvp3
// Slice Pattern Matching
//
// ferrocene-annotations: fls_nrkd5wpi64oo
// Literals
//
// ferrocene-annotations: fls_jkab8eevzbte
// Boolean Literals
//
// ferrocene-annotations: fls_d2sc9hl3v0mk
// Reference Patterns
//
// ferrocene-annotations: fls_org6hqv397fp
// Reference Pattern Matching
