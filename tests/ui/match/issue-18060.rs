//@ run-pass
// Regression test for #18060: match arms were matching in the wrong order.

#[allow(non_contiguous_range_endpoints)]
fn main() {
    assert_eq!(2, match (1, 3) { (0, 2..=5) => 1, (1, 3) => 2, (_, 2..=5) => 3, (_, _) => 4 });
    assert_eq!(2, match (1, 3) {                  (1, 3) => 2, (_, 2..=5) => 3, (_, _) => 4 });
    assert_eq!(2, match (1, 7) { (0, 2..=5) => 1, (1, 7) => 2, (_, 2..=5) => 3, (_, _) => 4 });
}

// ferrocene-annotations: fls_fyskeih6twyb
// Range Pattern Matching
//
// ferrocene-annotations: fls_6tl1fx99yn6c
// Range Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Wildcard Pattern Matching
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Wildcard Patterns
