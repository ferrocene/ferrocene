use std::array::IntoIter;

#[test]
fn exact_iterator_mut_ref_is_empty() {
    assert!((&mut (0..0)).is_empty());
    assert!(!(&mut (0..1)).is_empty());
}

#[test]
fn step_default_forward_and_backward() {
    use core::iter::Step;

    #[derive(Debug, Clone, PartialOrd, PartialEq)]
    struct Wrapper(usize);

    impl Step for Wrapper {
        fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
            usize::steps_between(&start.0, &end.0)
        }

        fn forward_checked(start: Self, count: usize) -> Option<Self> {
            usize::forward_checked(start.0, count).map(Self)
        }

        fn backward_checked(start: Self, count: usize) -> Option<Self> {
            usize::backward_checked(start.0, count).map(Self)
        }
    }

    assert_eq!(unsafe { Step::forward_unchecked(Wrapper(0), 10) }, Wrapper(10));
    assert_eq!(unsafe { Step::backward_unchecked(Wrapper(10), 10) }, Wrapper(0));
}

macro_rules! int_step {
    ($($T:ty => $fn:ident,)*) => {
        $(
            #[test]
            fn $fn() {
                let (lower_bound, upper_bound) = <$T as core::iter::Step>::steps_between(&<$T>::MIN, &<$T>::MAX);
                assert!(upper_bound.is_some() || lower_bound == usize::MAX);

                let (lower_bound, upper_bound) = <$T as core::iter::Step>::steps_between(&(<$T>::MAX / 2), &<$T>::MAX);
                assert!(upper_bound.is_some() || lower_bound == usize::MAX);

                let (lower_bound, upper_bound) = <$T as core::iter::Step>::steps_between(&(<$T>::MIN), &(<$T>::MIN + 1));
                assert_eq!(1, lower_bound);
                assert_eq!(Some(1), upper_bound);

                assert_eq!(
                    (0, None),
                    <$T as core::iter::Step>::steps_between(&<$T>::MAX, &<$T>::MIN)
                );

                assert_eq!(
                    <$T>::MAX,
                    <$T as core::iter::Step>::backward(<$T>::MIN, 1)
                );
                let half_max = (<$T>::MAX / 2) as usize;
                let expected = <$T>::MAX - half_max as $T;
                assert_eq!(
                    expected,
                    <$T as core::iter::Step>::backward(<$T>::MAX, half_max)
                );
                assert_eq!(
                    expected,
                    unsafe { <$T as core::iter::Step>::backward_unchecked(<$T>::MAX, half_max) }
                );
                assert_eq!(
                    Some(expected),
                    <$T as core::iter::Step>::backward_checked(<$T>::MAX, half_max)
                );
                assert!(
                    <$T as core::iter::Step>::backward_checked(<$T>::MIN, usize::MAX).is_none()
                );

                assert!(
                    <$T as core::iter::Step>::forward_checked(<$T>::MAX, usize::MAX).is_none()
                );
                assert_eq!(
                    Some(<$T>::MIN + <$T>::MAX as usize as $T),
                    <$T as core::iter::Step>::forward_checked(<$T>::MIN, <$T>::MAX as usize)
                );
                assert_eq!(
                    <$T>::MIN,
                    <$T as core::iter::Step>::forward(<$T>::MAX, 1)
                );
                assert_eq!(
                    <$T>::MIN + <$T>::MAX as usize as $T,
                    <$T as core::iter::Step>::forward(<$T>::MIN, <$T>::MAX as usize)
                );
            }
        )*
    };
}
int_step! {
    i8 => i8_step,
    i16 => i16_step,
    i32 => i32_step,
    i64 => i64_step,
    i128 => i128_step,
    isize => isize_step,
    u8 => u8_step,
    u16 => u16_step,
    u32 => u32_step,
    u64 => u64_step,
    u128 => u128_step,
    usize => usize_step,
}

#[test]
fn chunks_as_iter_nth() {
    let vals = [0, 1, 2, 3, 4, 5];
    let chunk = vals.chunks(2).nth(10);
    assert!(chunk.is_none());
}

// <core::iter::adapters::take::Take<I> as core::iter::traits::iterator::Iterator>::nth
#[test]
fn take_as_iter_nth() {
    let vals = [0, 1, 2, 3, 4, 5];
    let nth = vals.iter().take(2).nth(10);
    assert!(nth.is_none());
}

// <core::iter::adapters::skip::Skip<I> as core::iter::traits::iterator::Iterator>::fold
#[test]
fn skip_as_iter_fold() {
    let vals = [0, 1, 2, 3, 4, 5];
    let folded = vals.iter().skip(10).fold(0, |x, y| x + y);
    assert_eq!(folded, 0);
}

// <core::iter::adapters::skip::Skip<I> as core::iter::traits::iterator::Iterator>::try_fold
#[test]
fn skip_as_iter_try_fold() {
    let vals = [0, 1, 2, 3, 4, 5];
    let folded = vals.iter().skip(10).try_fold(0, |x, y| std::io::Result::Ok(x + y)).unwrap();
    assert_eq!(folded, 0);
}

// <A as core::iter::traits::iterator::SpecIterEq<B>>::spec_iter_eq
#[test]
fn spec_iter_eq() {
    let inf_1 = 0..;
    let inf_2 = 1..;
    assert_eq!(inf_1.into_iter().eq(inf_2), false);
}

// <core::char::decode::DecodeUtf16<I> as core::iter::traits::iterator::Iterator>::size_hint
// Basically `test_decode_utf16_size_hint` from `char.rs`
#[test]
fn decode_utf_16_as_iter_size_hint() {
    fn check(s: &[u16]) {
        let mut iter = char::decode_utf16(s.iter().cloned());

        loop {
            let count = iter.clone().count();
            let (lower, upper) = iter.size_hint();

            assert!(
                lower <= count && count <= upper.unwrap(),
                "lower = {lower}, count = {count}, upper = {upper:?}"
            );

            if let None = iter.next() {
                break;
            }
        }
    }

    check(&[0xD801, 0xD800, 0xD801, 0xD801]);
}

// <core::iter::adapters::chain::Chain<A, B> as core::iter::traits::iterator::Iterator>::advance_by
#[test]
fn test_iterator_chain_advance_by() {
    let first = vec![1, 2];
    let second = vec![4, 5];
    let mut iter = first.into_iter().chain(second.into_iter());
    iter.advance_back_by(3).unwrap(); // Make `self.b = None`
    iter.advance_by(2).ok(); // Go past `self.a`
    assert_eq!(iter.next(), None);
}

// <core::iter::adapters::zip::Zip<A, B> as core::iter::adapters::zip::ZipImpl<A, B>>::size_hint
#[test]
fn iter_zip_size_hint() {
    #[derive(Clone, Copy)]
    struct MaybeUpper {
        val: usize,
        size_hint: (usize, Option<usize>),
    }

    impl Iterator for MaybeUpper {
        type Item = usize;

        fn next(&mut self) -> Option<Self::Item> {
            Some(self.val)
        }
        fn size_hint(&self) -> (usize, Option<usize>) {
            self.size_hint
        }
    }

    let none = MaybeUpper { val: 1, size_hint: (0, None) };

    let some_1 = MaybeUpper { val: 1, size_hint: (1, Some(1)) };
    let some_2 = MaybeUpper { val: 2, size_hint: (2, Some(2)) };

    let none_none_zip = none.zip(none);
    assert_eq!(none_none_zip.size_hint(), (0, None));

    let some_some_zip = some_1.zip(some_2);
    assert_eq!(some_some_zip.size_hint(), (1, Some(1)));

    let some_none_zip = some_1.zip(none);
    assert_eq!(some_none_zip.size_hint(), (0, Some(1)));

    let none_some_zip = none.zip(some_2);
    assert_eq!(none_some_zip.size_hint(), (0, Some(2)));
}

// <core::iter::adapters::step_by::StepBy<I> as core::iter::adapters::step_by::StepByImpl<I>>::spec_nth
#[test]
fn iter_step_by_spec_nth() {
    let mut it = (0_u128..).step_by(1);
    let _first = it.next();
    let stepped = it.nth(usize::MAX);
    assert_eq!(stepped, Some(usize::MAX as u128 + 1));
}

// <core::slice::iter::Chunks<'a, T> as core::iter::traits::iterator::Iterator>::last
#[test]
fn iter_chunks_last() {
    let buf: Vec<usize> = vec![];
    let it = buf.chunks(5);
    assert_eq!(it.last(), None)
}

// <core::slice::iter::ChunksExact<'a, T> as core::iter::traits::iterator::Iterator>::nth
#[test]
fn iter_chunks_exact_nth() {
    let slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.chunks_exact(2);
    assert_eq!(iter.nth(55), None);
}

// <core::slice::iter::ChunksExactMut<'a, T> as core::iter::traits::iterator::Iterator>::nth
#[test]
fn iter_chunks_exact_mut_nth() {
    let mut slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.chunks_exact_mut(2);
    assert_eq!(iter.nth(55), None);
}

// <core::slice::iter::ChunksMut<'a, T> as core::iter::traits::iterator::Iterator>::last
#[test]
fn iter_chunks_mut_last() {
    let mut buf: Vec<usize> = vec![];
    let it = buf.chunks_mut(5);
    assert_eq!(it.last(), None)
}

// <core::slice::iter::ChunksMut<'a, T> as core::iter::traits::iterator::Iterator>::nth
#[test]
fn iter_chunks_mut_nth() {
    let mut slice = ['l', 'o', 'r', 'e', 'm'];
    let mut iter = slice.chunks_mut(2);
    assert_eq!(iter.nth(55), None);
}

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

// <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::advance_back_by
#[test]
fn test_iterator_copied_advance_back_by() {
    let first = vec![1, 2];
    let mut iter = first.iter().copied();
    iter.advance_back_by(1).ok();
    assert_eq!(iter.next(), Some(1));
}

// <core::iter::adapters::copied::Copied<I> as core::iter::traits::iterator::Iterator>::rfold
#[test]
fn test_iterator_copied_rfold() {
    let first = vec![1i8, 2];
    let iter = first.iter().copied();
    assert_eq!(iter.rfold(3, |x, acc| x - acc), 0);
}

// <core::iter::adapters::copied::Copied<I> as core::iter::traits::exact_size::ExactSizeIterator>::is_empty
#[test]
fn test_iterator_copied_is_empty() {
    let first = Vec::<u8>::new();
    let iter = first.iter().copied();
    assert!(iter.is_empty());
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

#[derive(Debug)]
struct IterWrapper<I>(I);

impl<I: Iterator> Iterator for IterWrapper<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

impl<I: DoubleEndedIterator> DoubleEndedIterator for IterWrapper<I> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back()
    }
}

// covers `<&mut I as core::iter::traits::iterator::IteratorRefSpec>::spec_try_fold`.
#[test]
fn test_spec_try_fold_for_mut_refs() {
    let x = [1_u16, 2, 3];
    let mut iter = IterWrapper(x.into_iter());
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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
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

// covers `core::iter::traits::double_ended::DoubleEndedIterator::nth_back`.
#[test]
fn test_double_ended_default_nth_back() {
    assert_eq!(None, IterWrapper(0..0).nth_back(10));
}

// covers `<core::iter::adapters::chain::Chain<A, B> as core::iter::traits::iterator::Iterator>::count`.
#[test]
fn test_chain_count_zero() {
    let mut iter = (0..2).chain(10..12);

    // ensure both fields a and b of Chain are None
    iter.nth(100);
    iter.nth_back(100);

    assert_eq!(0, iter.count());
}

// covers `<core::iter::adapters::take_while::TakeWhile<I, P> as core::iter::traits::iterator::Iterator>::size_hint`.
#[test]
fn test_take_while_size_hint_zero() {
    let mut iter = (0..2).take_while(|_| false);
    while iter.next().is_some() {}
    assert_eq!((0, Some(0)), iter.size_hint());
}

// covers `<core::iter::adapters::take_while::TakeWhile<I, P> as core::iter::traits::iterator::Iterator>::try_fold`.
#[test]
fn test_take_while_try_fold() {
    let mut iter = (0..2).take_while(|_| false);
    while iter.next().is_some() {}
    assert_eq!(Some(0), iter.try_fold(0, |a, b| Some(a + b)));
}

// covers `<core::iter::adapters::cloned::Cloned<I> as core::iter::traits::double_ended::DoubleEndedIterator>::rfold`.
#[test]
fn test_cloned_rfold() {
    let first = vec![1i8, 2];
    let iter = first.iter().cloned();
    assert_eq!(iter.rfold(3, |x, acc| x - acc), 0);
}

// covers `<core::iter::adapters::zip::Zip<A, B> as core::iter::adapters::zip::ZipFmt<A, B>>::fmt`.
#[test]
fn test_zip_debug_fmt() {
    let iter = vec![1u8, 2].into_iter().zip(vec![3u8, 4]);

    assert_eq!(format!("{iter:?}"), "Zip");

    #[derive(Debug)]
    struct Wrapper<I>(I);
    impl<I: Iterator> Iterator for Wrapper<I> {
        type Item = I::Item;

        fn next(&mut self) -> Option<Self::Item> {
            self.0.next()
        }
    }

    let iter = Wrapper(vec![1u8, 2].into_iter()).zip(Wrapper(vec![3u8, 4].into_iter()));

    assert_eq!(
        format!("{iter:?}"),
        "Zip { a: Wrapper(IntoIter([1, 2])), b: Wrapper(IntoIter([3, 4])) }"
    );
}

// covers `<core::iter::adapters::filter::Filter<I, P> as core::fmt::Debug>::fmt`.
#[test]
fn test_filter_debug_fmt() {
    let iter = vec![1u8, 2].into_iter().filter(|x| x % 2 == 0);

    assert_eq!(format!("{iter:?}"), "Filter { iter: IntoIter([1, 2]) }");
}

// covers `<core::iter::sources::from_fn::FromFn<F> as core::fmt::Debug>::fmt`.
#[test]
fn test_from_fn_debug_fmt() {
    let mut x = 0;
    let iter = core::iter::from_fn(|| {
        x += 1;
        Some(x)
    });

    assert_eq!(format!("{iter:?}"), "FromFn");
}

// covers `<core::iter::adapters::map::Map<I, F> as core::fmt::Debug>::fmt`.
#[test]
fn test_map_debug_fmt() {
    let iter = vec![1u8, 2].into_iter().map(|x| x + 1);

    assert_eq!(format!("{iter:?}"), "Map { iter: IntoIter([1, 2]) }");
}

// covers `<core::iter::adapters::flatten::FlatMap<I, U, F> as core::fmt::Debug>::fmt`.
#[test]
fn test_flat_map_debug_fmt() {
    let iter = vec![vec![1u8, 2]].into_iter().flat_map(|x| x);

    assert_eq!(
        format!("{iter:?}"),
        "FlatMap { inner: FlattenCompat { iter: Fuse { iter: Some(Map { iter: IntoIter([[1, 2]]) }) }, frontiter: None, backiter: None } }"
    );
}

// covers `<core::iter::adapters::take_while::TakeWhile<I, P> as core::fmt::Debug>::fmt`.
#[test]
fn test_take_while_debug_fmt() {
    let iter = vec![1u8, 2].into_iter().take_while(|_| true);

    assert_eq!(format!("{iter:?}"), "TakeWhile { iter: IntoIter([1, 2]), flag: false }");
}
