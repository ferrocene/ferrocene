// <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::advance_by
#[test]
fn test_iterator_copied_advance_by() {
    let first = vec![1, 2];
    let mut iter = first.iter().copied();
    iter.advance_by(1).ok(); // Go past `self.a`
    assert_eq!(iter.next(), Some(2));
}
