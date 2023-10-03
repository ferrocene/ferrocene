#![deny(unreachable_patterns)]

fn main() {
    let s: &[bool] = &[];

    match s {
        [true, ..] => {}
        [true, ..] => {} //~ ERROR unreachable pattern
        [true] => {} //~ ERROR unreachable pattern
        [..] => {}
    }
    match s {
        [.., true] => {}
        [.., true] => {} //~ ERROR unreachable pattern
        [true] => {} //~ ERROR unreachable pattern
        [..] => {}
    }
    match s {
        [false, .., true] => {}
        [false, .., true] => {} //~ ERROR unreachable pattern
        [false, true] => {} //~ ERROR unreachable pattern
        [false] => {}
        [..] => {}
    }
}

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
