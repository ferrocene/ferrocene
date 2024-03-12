//@ check-pass

fn main() {
    for _ in [1, 2] {}
    let x = [1, 2];
    for _ in x {}
    for _ in [1.0, 2.0] {}
}

// ferrocene-annotations: fls_xinykul167l
// Array Expressions
// ferrocene-annotations: fls_onfyolkcbeh3
// For Loops
