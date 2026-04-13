#[test]
fn test_assert_matches_success() {
    let a = Some(12);

    assert!(matches!(a, Some(_)));
    assert!(matches!(a, Some(12)));
    assert!(matches!(a, Some(12) | None));
}

// Covers:
// - `core::panicking::assert_matches_failed`
// - <core::panicking::assert_matches_failed::Pattern<'_> as core::fmt::Debug>::fmt
#[test]
#[should_panic = "assertion failed"]
fn test_assert_matches_failure_some() {
    let a = Some(12);
    assert!(matches!(a, Some(34)));
}

// Covers:
// - `core::panicking::assert_matches_failed`
// - <core::panicking::assert_matches_failed::Pattern<'_> as core::fmt::Debug>::fmt
#[test]
#[should_panic = "assertion failed"]
fn test_assert_matches_failure_none() {
    let a = Some(12);
    assert!(matches!(a, None));
}

// Covers `core::panicking::assert_failed_inner`
#[test]
#[should_panic = "assertion `left != right` failed\n  left: 5\n right: 5"]
fn test_assert_failed_inner_ne() {
    assert_ne!(5, 5);
}
