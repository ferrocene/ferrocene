//@ run-pass

fn main () {
    let n = (1, (2, 3)).1.1;
    assert_eq!(n, 3);

    let n = (1, (2, (3, 4))).1.1.1;
    assert_eq!(n, 4);

    // This is a range expression, not nested indexing.
    let _ = 0.0..1.1;
}

// ferrocene-annotations: fls_18swodqqzadj
// Range Expressions
// ferrocene-annotations: fls_k64tfywtn0g8
// Tuple Expressions
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
