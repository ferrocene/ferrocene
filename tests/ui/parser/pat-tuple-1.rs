fn main() {
    match (0, 1) {
        (, ..) => {} //~ ERROR expected pattern, found `,`
    }
}

// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
