pub fn main() {
    let sl: &[u8] = b"foo";

    match sl { //~ ERROR non-exhaustive patterns
        [first, remainder @ ..] => {},
    };
}

// ferrocene-annotations: fls_qte70mgzpras
// Slice Patterns
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
