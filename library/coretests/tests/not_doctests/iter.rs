use std::array::IntoIter;

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

// <core::iter::adapters::rev::Rev<I> as core::iter::traits::double_ended::DoubleEndedIterator>::rfind
#[test]
fn test_iter_rev_double_ended_rfind() {
    let val = vec![1, 2, 3, 4];
    let iter = val.iter();
    let mut rev = iter.rev();
    assert_eq!(*rev.rfind(|_| true).unwrap(), 1);
}

// <core::iter::adapters::rev::Rev<I> as core::iter::traits::double_ended::DoubleEndedIterator>::rfold
#[test]
fn test_iter_rev_double_ended_rfold() {
    let val = vec![1, 2, 3];
    let iter = val.iter();
    let rev = iter.rev();
    assert_eq!(rev.rfold(0, |a, b| a + b), 6);
}

// <core::iter::adapters::zip::Zip<A, B> as core::iter::adapters::zip::ZipImpl<A, B>>::fold
#[test]
fn test_iter_zip_fold() {
    let first = vec![1, 2, 3];
    let first_iter: Box<dyn Iterator<Item = i32>> = Box::new(first.into_iter());
    let second = vec![1, 2, 3];
    let second_iter: Box<dyn Iterator<Item = i32>> = Box::new(second.into_iter());
    let zipped: core::iter::Zip<_, _> = first_iter.zip(second_iter);
    assert_eq!(zipped.fold(0, |a, (b1, b2)| a + b1 + b2), 12);
}

// <core::iter::adapters::zip::Zip<A, B> as core::iter::adapters::zip::ZipImpl<A, B>>::nth
#[test]
fn test_iter_zip_nth() {
    let first = vec![1, 2, 3];
    let first_iter: Box<dyn Iterator<Item = i32>> = Box::new(first.into_iter());
    let second = vec![1, 2, 3];
    let second_iter: Box<dyn Iterator<Item = i32>> = Box::new(second.into_iter());
    let mut zipped: core::iter::Zip<_, _> = first_iter.zip(second_iter);
    assert_eq!(zipped.nth(1), Some((2, 2)));
}

// <core::iter::adapters::rev::Rev<I> as core::iter::traits::iterator::Iterator>::find
#[test]
fn test_iter_rev_find() {
    let first = vec![1, 2, 3];
    let mut reversed = first.iter().rev();
    assert_eq!(*reversed.find(|_| true).unwrap(), 3);
}

// <&mut I as core::iter::traits::iterator::Iterator>::advance_by
#[test]
fn test_mut_ref_iterator_advance_by() {
    let x = &mut [1, 2, 3];
    let mut iter = x.iter();
    assert!(Iterator::advance_by(&mut &mut iter, 1).is_ok());
}

// <core::array::iter::IntoIter<T, N> as core::iter::traits::double_ended::DoubleEndedIterator>::try_rfold
#[test]
fn test_array_into_iter_double_ended_rfold() {
    let x = [1_u16, 2, 3];
    let mut iter = x.into_iter();
    assert!(
        <IntoIter<_, _> as DoubleEndedIterator>::try_rfold(&mut &mut iter, 0_u16, |a, b| a
            .checked_add(b))
        .is_some()
    );
}

// <core::iter::adapters::step_by::StepBy<core::ops::range::Range<u16>> as core::iter::adapters::step_by::StepByImpl<core::ops::range::Range<u16>>>::spec_nth
#[test]
fn test_iter_step_by_spec_try_fold() {
    let x = 0_u16..100;
    let iter = x.into_iter();
    let mut step_by = iter.step_by(2);
    assert_eq!(
        step_by.nth(2),
        Some(4),
    );
}
