fn foo(a: Option<usize>, b: Option<usize>) {
    match (a, b) {
        //~^ ERROR: non-exhaustive patterns: `(None, None)` and `(Some(_), Some(_))` not covered
        (Some(a), Some(b)) if a == b => {}
        (Some(_), None) | (None, Some(_)) => {}
    }
}

fn main() {
    foo(None, None);
}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
