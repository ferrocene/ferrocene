//@ check-pass

fn main() {
    match (0, 1, 2) {
        (pat, ..,) => {}
    }
}

// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
