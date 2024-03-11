//@ run-pass

pub fn main() {
    let x = 1;
    let y = 2;

    assert_eq!(3, match (x, y) {
        (1, 1) => 1,
        (2, 2) => 2,
        (1..=2, 2) => 3,
        _ => 4,
    });

    // nested tuple
    assert_eq!(3, match ((x, y),) {
        ((1, 1),) => 1,
        ((2, 2),) => 2,
        ((1..=2, 2),) => 3,
        _ => 4,
    });
}

// ferrocene-annotations: fls_azzf1llv3wf
// Literal Pattern Matching
//
// ferrocene-annotations: fls_2krxnq8q9ef1
// Literal Patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
