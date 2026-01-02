// <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::advance_by
#[test]
fn test_iterator_copied_advance_by() {
    let first = vec![1, 2];
    let mut iter = first.iter().copied();
    iter.advance_by(1).ok();
    assert_eq!(iter.next(), Some(2));
}

// <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::last
#[test]
fn test_iterator_copied_last() {
    let first = vec![1, 2];
    let iter = first.iter().copied();
    assert_eq!(iter.last(), Some(2));
}

// <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::nth
#[test]
fn test_iterator_copied_nth() {
    let first = vec![1, 2];
    let mut iter = first.iter().copied();
    assert_eq!(iter.nth(1), Some(2));
}
