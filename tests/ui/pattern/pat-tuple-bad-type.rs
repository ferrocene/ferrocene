fn main() {
    let x; //~ ERROR type annotations needed

    match x {
        (..) => {}
        _ => {}
    }

    match 0u8 {
        (..) => {} //~ ERROR mismatched types
        _ => {}
    }

    x = 10;
}

// ferrocene-annotations: fls_5loglxds6zik
// Parenthesized Pattern Matching
//
// ferrocene-annotations: fls_1xit18et4ohh
// Parenthesized Patterns
