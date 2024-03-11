//@ check-pass

fn main() {
    let s: &[bool] = &[true; 0];
    let s0: &[bool; 0] = &[];
    let s1: &[bool; 1] = &[false; 1];
    let s2: &[bool; 2] = &[false; 2];

    let [] = s0;
    let [_] = s1;
    let [_, _] = s2;

    let [..] = s;
    let [..] = s0;
    let [..] = s1;
    let [..] = s2;

    let [_, ..] = s1;
    let [.., _] = s1;
    let [_, ..] = s2;
    let [.., _] = s2;

    let [_, _, ..] = s2;
    let [_, .., _] = s2;
    let [.., _, _] = s2;
}

// ferrocene-annotations: fls_uh76pw6ykd57
// Refutability
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_vpbikb73dw4k
// Slice Types
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_nrkd5wpi64oo
// Literals
//
// ferrocene-annotations: fls_jkab8eevzbte
// Boolean Literals
//
// ferrocene-annotations: fls_hv9jtycp0o1y
// Numeric Literals
//
// ferrocene-annotations: fls_2ed4axpsy9u0
// Integer Literals
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_qte70mgzpras
// Slice Patterns
//
// ferrocene-annotations: fls_57ic33pwdvp3
// Slice Pattern Matching
//
// ferrocene-annotations: fls_7wpgnp4kjq82
// Rest Patterns
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Underscore Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Underscore Pattern Matching
