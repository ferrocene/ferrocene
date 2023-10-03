fn main() {
    match (0, 1, 2) {
        (.., pat, ..) => {}
        //~^ ERROR `..` can only be used once per tuple pattern
    }
}

// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
