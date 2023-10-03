#![feature(half_open_range_patterns_in_slices)]

fn main() {
    match [1, 2] {
        [a.., a] => {} //~ ERROR cannot find value `a` in this scope
    }
}

// ferrocene-annotations: fls_57ic33pwdvp3
// Slice Pattern Matching
//
// ferrocene-annotations: fls_qte70mgzpras
// Slice Patterns
