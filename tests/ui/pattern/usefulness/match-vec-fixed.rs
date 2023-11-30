#![deny(unreachable_patterns)]

fn a() {
    let v = [1, 2, 3];
    match v {
        [_, _, _] => {}
        [_, _, _] => {} //~ ERROR unreachable pattern
    }
    match v {
        [_, 1, _] => {}
        [_, 1, _] => {} //~ ERROR unreachable pattern
        _ => {}
    }
}

fn main() {
    a();
}

// ferrocene-annotations: fls_uj0kpjwyld60
// Array Types
//
// ferrocene-annotations: fls_xinykul167l
// Array Expressions
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
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
// ferrocene-annotations: fls_qfsfnql1t7m
// Underscore Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Underscore Pattern Matching
