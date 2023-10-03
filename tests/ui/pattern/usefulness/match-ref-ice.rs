#![deny(unreachable_patterns)]

// The arity of `ref x` is always 1. If the pattern is compared to some non-structural type whose
// arity is always 0, an ICE occurs.
//
// Related issue: #23009

fn main() {
    let homura = [1, 2, 3];

    match homura {
        [1, ref _madoka, 3] => (),
        [1, 2, 3] => (), //~ ERROR unreachable pattern
        [_, _, _] => (),
    }
}

// ferrocene-annotations: fls_xinykul167l
// Array Expressions
//
// ferrocene-annotations: fls_uj0kpjwyld60
// Array Types
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
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
