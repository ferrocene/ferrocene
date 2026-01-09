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
    assert_eq!(step_by.nth(2), Some(4),);
}

// Used to test Range bits.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
enum Steppable {
    A,
    B,
    C,
}
impl core::iter::Step for Steppable {
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        match (start, end) {
            (Self::A, Self::A) => (0, Some(0)),
            (Self::A, Self::B) => (1, Some(1)),
            (Self::A, Self::C) => (2, Some(2)),
            (Self::B, Self::B) => (0, Some(0)),
            (Self::B, Self::C) => (1, Some(1)),
            (Self::C, Self::C) => (0, Some(0)),
            (Self::C, Self::B) => (1, Some(1)),
            (Self::C, Self::A) => (2, Some(2)),
            (Self::B, Self::A) => (1, Some(1)),
        }
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        match start {
            Self::A => match count {
                0 => Some(Self::A),
                1 => Some(Self::B),
                2 => Some(Self::C),
                _ => None,
            },
            Self::B => match count {
                0 => Some(Self::B),
                1 => Some(Self::C),
                _ => None,
            },
            Self::C => match count {
                0 => Some(Self::C),
                _ => None,
            },
        }
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        match start {
            Self::A => match count {
                0 => Some(Self::A),
                _ => None,
            },
            Self::B => match count {
                0 => Some(Self::B),
                1 => Some(Self::A),
                _ => None,
            },
            Self::C => match count {
                0 => Some(Self::C),
                1 => Some(Self::B),
                2 => Some(Self::A),
                _ => None,
            },
        }
    }
}

// Used to test Range bits.
#[derive(Clone, PartialEq, PartialOrd, Debug)]
enum SteppableBrokenStepsBetween {
    A,
    B,
    C,
}

impl core::iter::Step for SteppableBrokenStepsBetween {
    fn steps_between(_start: &Self, _end: &Self) -> (usize, Option<usize>) {
        (1, None)
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        if count == 1 {
            return None;
        }
        match start {
            Self::A => match count {
                0 => Some(Self::A),
                1 => Some(Self::B),
                2 => Some(Self::C),
                _ => None,
            },
            Self::B => match count {
                0 => Some(Self::B),
                1 => Some(Self::C),
                _ => None,
            },
            Self::C => match count {
                0 => Some(Self::C),
                _ => None,
            },
        }
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        if count == 1 {
            return None;
        }
        match start {
            Self::A => match count {
                0 => Some(Self::A),
                _ => None,
            },
            Self::B => match count {
                0 => Some(Self::B),
                1 => Some(Self::A),
                _ => None,
            },
            Self::C => match count {
                0 => Some(Self::C),
                1 => Some(Self::B),
                2 => Some(Self::A),
                _ => None,
            },
        }
    }
}

// <core::ops::range::Range<A> as core::iter::range::RangeIteratorImpl>::spec_nth
#[test]
fn test_range_spec_nth() {
    let mut x = core::ops::Range { start: Steppable::A, end: Steppable::C };
    assert_eq!(<core::ops::Range<Steppable> as Iterator>::nth(&mut x, 2), None,);
}

// <core::ops::range::Range<A> as core::iter::range::RangeIteratorImpl>::spec_nth_back
#[test]
fn test_range_spec_nth_back() {
    let mut x = core::ops::Range { start: Steppable::A, end: Steppable::C };
    assert_eq!(
        <core::ops::Range<Steppable> as DoubleEndedIterator>::nth_back(&mut x, 1),
        Some(Steppable::A),
    );
    assert_eq!(<core::ops::Range<Steppable> as DoubleEndedIterator>::nth_back(&mut x, 4), None,);
}

// <core::ops::range::Range<A> as core::iter::range::RangeIteratorImpl>::spec_nth
#[test]
#[should_panic = "`Step` invariants not upheld"]
fn test_range_spec_nth_invariant() {
    let mut x = core::ops::Range {
        start: SteppableBrokenStepsBetween::A,
        end: SteppableBrokenStepsBetween::C,
    };
    assert_eq!(<core::ops::Range<SteppableBrokenStepsBetween> as Iterator>::nth(&mut x, 0), None,);
}

// <core::ops::range::Range<A> as core::iter::range::RangeIteratorImpl>::spec_advance_by
#[test]
#[should_panic = "`Step` invariants not upheld"]
fn test_range_spec_advance_by() {
    let mut x = core::ops::Range {
        start: SteppableBrokenStepsBetween::A,
        end: SteppableBrokenStepsBetween::C,
    };
    assert!(
        <core::ops::Range<SteppableBrokenStepsBetween> as Iterator>::advance_by(&mut x, 4).is_err()
    );
}

// <core::ops::range::Range<A> as core::iter::range::RangeIteratorImpl>::spec_advance_back_by
#[test]
fn test_range_spec_advance_back_by() {
    let mut x = core::ops::Range { start: Steppable::A, end: Steppable::C };
    assert!(
        <core::ops::Range<Steppable> as DoubleEndedIterator>::advance_back_by(&mut x, 4).is_err()
    );
}

// <core::ops::range::Range<A> as core::iter::range::RangeIteratorImpl>::spec_next
#[test]
fn test_range_spec_next() {
    let mut x = core::ops::Range { start: Steppable::A, end: Steppable::C };
    assert_eq!(<core::ops::Range<Steppable> as Iterator>::next(&mut x), Some(Steppable::A),);
    assert_eq!(<core::ops::Range<Steppable> as Iterator>::next(&mut x), Some(Steppable::B),);
    assert_eq!(<core::ops::Range<Steppable> as Iterator>::next(&mut x), None,);
}

// <core::ops::range::Range<A> as core::iter::range::RangeIteratorImpl>::spec_next_back
#[test]
fn test_range_spec_next_back() {
    let mut x = core::ops::Range { start: Steppable::A, end: Steppable::C };
    assert_eq!(
        <core::ops::Range<Steppable> as DoubleEndedIterator>::next_back(&mut x),
        Some(Steppable::B),
    );
    assert_eq!(
        <core::ops::Range<Steppable> as DoubleEndedIterator>::next_back(&mut x),
        Some(Steppable::A),
    );
    assert_eq!(<core::ops::Range<Steppable> as DoubleEndedIterator>::next_back(&mut x), None,);
}

// <I as core::iter::traits::iterator::Iterator::advance_by::SpecAdvanceBy>::spec_advance_by
#[test]
fn test_iterator_spec_advance_by() {
    struct FooSome;
    trait FunSome {}
    impl FunSome for FooSome {}
    impl Iterator for dyn FunSome {
        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            Some(1)
        }
    }

    let mut foo = FooSome;
    let foo: &mut dyn FunSome = &mut foo;
    assert!(foo.advance_by(5).is_ok());

    struct FooNone;
    trait FunNone {}
    impl FunNone for FooNone {}
    impl Iterator for dyn FunNone {
        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            None
        }
    }

    let mut foo = FooNone;
    let foo: &mut dyn FunNone = &mut foo;
    assert!(foo.advance_by(5).is_err());
}

// <&mut I as core::iter::traits::iterator::IteratorRefSpec>::spec_try_fold
#[test]
fn test_iterator_spec_try_fold() {
    struct FooSome;
    trait FunSome {}
    impl FunSome for FooSome {}
    impl Iterator for dyn FunSome {
        type Item = usize;
        fn next(&mut self) -> Option<Self::Item> {
            Some(1)
        }
    }

    use core::ops::ControlFlow;
    let mut foo = FooSome;
    let foo: &mut dyn FunSome = &mut foo;
    let mut took = foo.take(10);
    assert!(took.try_fold(0, |a, b| ControlFlow::<usize, _>::Continue(a + b)).is_continue());
    assert!(took.try_fold(0, |_, _| ControlFlow::Break(1)).is_continue());
}

// <core::slice::iter::Windows<'a, T> as core::iter::traits::iterator::Iterator>::nth
#[test]
fn test_iterator_windows_nth() {
    let x = [1, 2, 3, 4, 5, 6];
    let mut windows = x.windows(2);
    assert_eq!(Iterator::nth(&mut windows, 55), None);
}

// <core::slice::iter::Windows<'a, T> as core::iter::traits::iterator::Iterator>::last
#[test]
fn test_iterator_windows_last() {
    let x: [usize; 0] = [];
    let windows = x.windows(2);
    assert_eq!(windows.last(), None);
}

// <core::str::iter::Chars<'a> as core::iter::traits::iterator::Iterator>::advance_by
#[test]
fn test_chars_iter_advance_by() {
    macro_rules! repeat8 {
        ($s:expr) => {
            concat!($s, $s, $s, $s, $s, $s, $s, $s)
        };
    }
    const CORPORA: &str = repeat8!(
        "Сотни компаний по всему миру используют Rust в реальных\
        проектах для быстрых кросс-платформенных решений с\
        ограниченными ресурсами. Такие проекты, как Firefox,\
        Dropbox и Cloudflare, используют Rust. Rust отлично\
        подходит как для стартапов, так и для больших компаний,\
        как для встраиваемых устройств, так и для масштабируемых\
        web-сервисов. Мой самый большой комплимент Rust."
    );
    for x in 0..1000 {
        let mut chars = CORPORA.chars();
        chars.advance_by(x).ok();
    }
}

// core::slice::iter::IterMut::<'a, T>::as_mut_slice
#[test]
fn test_iter_mut_as_mut_slice() {
    let mut x = [1, 2, 3];
    let mut iter = x.iter_mut();
    assert_eq!([1, 2, 3], iter.as_mut_slice());
}

#[test]
fn test_index_range_double_ended_iterator() {
    core::ferrocene_test::test_index_range_double_ended_iterator();
}

#[test]
fn test_index_range_iterator() {
    core::ferrocene_test::test_index_range_iterator();
}

// cover <core::slice::iter::ChunksMut<'a, T> as core::iter::traits::iterator::Iterator>::nth
#[test]
fn test_iterator_chunksmut_nth() {
    let iter = &mut [(); usize::MAX];
    let mut chunked = iter.chunks_mut(100);
    assert!(chunked.nth(usize::MAX / 100).unwrap().len() < 100);
}

// covers `<core::iter::adapters::chain::Chain<A, B> as core::iter::traits::iterator::Iterator>::nth`.
#[test]
fn test_nth_for_chain() {
    let a = vec![].into_iter();
    let b = vec![1, 2, 3].into_iter();

    let mut iter = a.chain(b);

    assert_eq!(Some(1), iter.nth(0));
    assert_eq!(Some(3), iter.nth(1));
}

// covers `<core::iter::adapters::skip::Skip<I> as core::iter::traits::iterator::Iterator>::try_fold`.
#[test]
fn test_try_fold_for_skip() {
    let mut iter = vec![1i32, 2, 3].into_iter().skip(0);
    assert!(iter.try_fold(0i32, |a, b| a.checked_add(b)).is_some());
}

// covers `<&mut I as core::iter::traits::iterator::IteratorRefSpec>::spec_try_fold`.
#[test]
fn test_spec_try_fold_for_mut_refs() {
    struct Wrapper<I>(I);

    impl<I: Iterator> Iterator for Wrapper<I> {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    let x = [1_u16, 2, 3];
    let mut iter = Wrapper(x.into_iter());
    let mut iter_ref = &mut iter as &mut dyn Iterator<Item = u16>;

    assert!(
        <&mut dyn Iterator<Item = u16> as Iterator>::try_fold(&mut iter_ref, 0_u16, |a, b| a
            .checked_add(b))
        .is_some()
    );
}

// covers `<core::iter::adapters::step_by::StepBy<I> as core::iter::adapters::step_by::StepByImpl<I>>::spec_fold`.
#[test]
fn test_spec_fold_for_step_by() {
    assert_eq!(25, (1_u16..=10).step_by(2).fold(0_u16, |a, b| a + b));
    assert_eq!(24, (1_u16..=10).step_by(2).skip(1).fold(0_u16, |a, b| a + b));

    assert_eq!(0, Option::<u16>::None.into_iter().step_by(2).fold(0_u16, |a, b| a + b));
}

// covers `<core::iter::adapters::step_by::StepBy<core::ops::range::Range<u16>> as core::iter::adapters::step_by::StepByImpl<core::ops::range::Range<u16>>>::spec_try_fold`.
#[test]
fn test_spec_try_fold_for_step_by_range_u16() {
    assert_eq!(Some(25), (1_u16..10).step_by(2).try_fold(0_u16, |a, b| a.checked_add(b)));
}

#[derive(Clone, PartialOrd, PartialEq)]
struct StepWrapper(u16);

impl core::iter::Step for StepWrapper {
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        u16::steps_between(&start.0, &end.0)
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        u16::forward_checked(start.0, count).map(Self)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        u16::backward_checked(start.0, count).map(Self)
    }
}

// covers `<core::ops::range::Range<A> as core::iter::range::RangeIteratorImpl>::spec_nth`.
#[test]
fn test_spec_nth_for_range() {
    assert_eq!(Some(StepWrapper(2)), (StepWrapper(1)..StepWrapper(10)).nth(1));
    assert_eq!(None, (StepWrapper(1)..StepWrapper(10)).nth(10));
    assert_eq!(None, (StepWrapper(1)..StepWrapper(10)).nth(usize::MAX));
}

// covers `<core::ops::range::Range<A> as core::iter::range::RangeIteratorImpl>::spec_nth_back`.
#[test]
fn test_spec_nth_back_for_range() {
    assert_eq!(Some(StepWrapper(8)), (StepWrapper(1)..StepWrapper(10)).nth_back(1));
    assert_eq!(None, (StepWrapper(1)..StepWrapper(10)).nth_back(10));
    assert_eq!(None, (StepWrapper(1)..StepWrapper(10)).nth_back(usize::MAX));
}

// covers `<core::ops::range::RangeInclusive<A> as core::iter::range::RangeInclusiveIteratorImpl>::spec_next`.
#[test]
fn test_spec_next_for_range_inclusive() {
    assert_eq!(Some(StepWrapper(1)), (StepWrapper(1)..=StepWrapper(1)).next());
    assert_eq!(Some(StepWrapper(1)), (StepWrapper(1)..=StepWrapper(2)).next());
    assert_eq!(None, (StepWrapper(2)..=StepWrapper(1)).next());
}

// covers `<core::ops::range::RangeInclusive<A> as core::iter::range::RangeInclusiveIteratorImpl>::spec_next_back`.
#[test]
fn test_spec_next_back_for_range_inclusive() {
    assert_eq!(Some(StepWrapper(1)), (StepWrapper(1)..=StepWrapper(1)).next_back());
    assert_eq!(Some(StepWrapper(2)), (StepWrapper(1)..=StepWrapper(2)).next_back());
    assert_eq!(None, (StepWrapper(2)..=StepWrapper(1)).next_back());
}

#[derive(Clone, PartialOrd, PartialEq)]
struct DoubleStepWrapper(u16);

impl core::iter::Step for DoubleStepWrapper {
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        if *start <= *end {
            let steps = (end.0 - start.0) as usize / 2;
            (steps, Some(steps))
        } else {
            (0, None)
        }
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        u16::forward_checked(start.0, 2 * count).map(Self)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        u16::backward_checked(start.0, 2 * count).map(Self)
    }
}

// covers `<core::ops::range::RangeInclusive<A> as core::iter::range::RangeInclusiveIteratorImpl>::spec_try_fold`.
#[test]
fn test_spec_try_fold_for_range_inclusive() {
    assert_eq!(
        Some(StepWrapper(55)),
        (StepWrapper(1)..=StepWrapper(10))
            .try_fold(StepWrapper(0), |a, b| a.0.checked_add(b.0).map(StepWrapper))
    );

    assert_eq!(
        Some(StepWrapper(0)),
        (StepWrapper(2)..=StepWrapper(1))
            .try_fold(StepWrapper(0), |a, b| a.0.checked_add(b.0).map(StepWrapper))
    );

    assert_eq!(
        Some(StepWrapper(1)),
        (StepWrapper(1)..=StepWrapper(1))
            .try_fold(StepWrapper(0), |a, b| a.0.checked_add(b.0).map(StepWrapper))
    );

    assert!(
        (DoubleStepWrapper(1)..=DoubleStepWrapper(10))
            .try_fold(DoubleStepWrapper(0), |a, b| a.0.checked_add(b.0).map(DoubleStepWrapper))
            .is_some()
    );
}

// covers `<core::ops::range::RangeInclusive<A> as core::iter::range::RangeInclusiveIteratorImpl>::spec_try_rfold`.
#[test]
fn test_spec_try_rfold_for_range_inclusive() {
    assert_eq!(
        Some(StepWrapper(55)),
        (StepWrapper(1)..=StepWrapper(10))
            .try_rfold(StepWrapper(0), |a, b| a.0.checked_add(b.0).map(StepWrapper))
    );

    assert_eq!(
        Some(StepWrapper(0)),
        (StepWrapper(2)..=StepWrapper(1))
            .try_rfold(StepWrapper(0), |a, b| a.0.checked_add(b.0).map(StepWrapper))
    );

    assert_eq!(
        Some(StepWrapper(1)),
        (StepWrapper(1)..=StepWrapper(1))
            .try_rfold(StepWrapper(0), |a, b| a.0.checked_add(b.0).map(StepWrapper))
    );

    assert!(
        (DoubleStepWrapper(1)..=DoubleStepWrapper(10))
            .try_rfold(DoubleStepWrapper(0), |a, b| a.0.checked_add(b.0).map(DoubleStepWrapper))
            .is_some()
    );
}

#[derive(Clone, Copy, PartialOrd, PartialEq)]
struct TrustedDoubleStepWrapper(u16);

impl core::iter::Step for TrustedDoubleStepWrapper {
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        if *start <= *end {
            let steps = (end.0 - start.0) as usize / 2;
            (steps, Some(steps))
        } else {
            (0, None)
        }
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        u16::forward_checked(start.0, 2 * count).map(Self)
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        u16::backward_checked(start.0, 2 * count).map(Self)
    }
}

unsafe impl core::iter::TrustedStep for TrustedDoubleStepWrapper {}

// covers `<core::ops::range::RangeInclusive<T> as core::iter::range::RangeInclusiveIteratorImpl>::spec_try_fold`.
#[test]
fn test_spec_try_fold_for_trusted_range_inclusive() {
    assert!(
        (TrustedDoubleStepWrapper(1)..=TrustedDoubleStepWrapper(10))
            .try_fold(TrustedDoubleStepWrapper(0), |a, b| a
                .0
                .checked_add(b.0)
                .map(TrustedDoubleStepWrapper))
            .is_some()
    );
}

// covers `<core::ops::range::RangeInclusive<T> as core::iter::range::RangeInclusiveIteratorImpl>::spec_try_rfold`.
#[test]
fn test_spec_try_rfold_for_trusted_range_inclusive() {
    assert!(
        (TrustedDoubleStepWrapper(1)..=TrustedDoubleStepWrapper(10))
            .try_rfold(TrustedDoubleStepWrapper(0), |a, b| a
                .0
                .checked_add(b.0)
                .map(TrustedDoubleStepWrapper))
            .is_some()
    );
}

// covers `<core::slice::iter::Iter<'a, T> as core::iter::traits::double_ended::DoubleEndedIterator>::nth_back`.
#[test]
fn test_nth_back_for_slice_iter() {
    let mut arr = [(); 10];
    let slice = arr.as_mut_slice();

    assert_eq!(None, slice.iter().nth_back(11));
}

// covers `core::iter::range::<impl core::iter::traits::double_ended::DoubleEndedIterator for core::ops::range::RangeInclusive<A>>::nth_back`.
#[test]
fn test_nth_back_for_range_inclusive() {
    assert_eq!(None, (1..=0).nth_back(11));
    assert_eq!(None, (DoubleStepWrapper(1)..=DoubleStepWrapper(1)).nth_back(1));
}
