fn main() {
    match 0 { 1 => () } //~ ERROR non-exhaustive patterns
    match 0 { 0 if false => () } //~ ERROR non-exhaustive patterns
}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_2krxnq8q9ef1
// Literal Patterns
//
// ferrocene-annotations: fls_azzf1llv3wf
// Literal Pattern Matching
