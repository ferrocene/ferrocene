//@ run-pass
// Test that we "reset" the mode as we pass through a `&` pattern.
//
// cc #46688

fn surprise(x: i32) {
    assert_eq!(x, 2);
}

fn main() {
    let x = &(1, &2);
    let (_, &b) = x;
    surprise(b);
}

// ferrocene-annotations: fls_org6hqv397fp
// Reference Pattern Matching
//
// ferrocene-annotations: fls_d2sc9hl3v0mk
// Reference Patterns
