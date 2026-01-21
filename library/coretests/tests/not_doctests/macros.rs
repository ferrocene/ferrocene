use core::assert_matches;

#[test]
fn test_assert_matches_success() {
    let a = Some(12);

    assert_matches!(a, Some(_));
    assert_matches!(a, Some(12));
    assert_matches!(a, Some(12) | None);
}

// Covers:
// - `core::panicking::assert_matches_failed`
// - <core::panicking::assert_matches_failed::Pattern<'_> as core::fmt::Debug>::fmt
#[test]
#[should_panic = "assertion `left matches right` failed\n  left: Some(12)\n right: Some(34)"]
fn test_assert_matches_failure_some() {
    let a = Some(12);
    assert_matches!(a, Some(34));
}

// Covers:
// - `core::panicking::assert_matches_failed`
// - <core::panicking::assert_matches_failed::Pattern<'_> as core::fmt::Debug>::fmt
#[test]
#[should_panic = "assertion `left matches right` failed\n  left: Some(12)\n right: None"]
fn test_assert_matches_failure_none() {
    let a = Some(12);
    assert_matches!(a, None);
}
