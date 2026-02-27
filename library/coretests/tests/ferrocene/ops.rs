use core::ops::{Bound, ControlFlow};

#[test]
fn control_flow_is_methods() {
    let c = ControlFlow::<i32, i32>::Continue(0);
    assert!(c.is_continue());
    assert!(!c.is_break());

    let b = ControlFlow::<i32, i32>::Break(1);
    assert!(!b.is_continue());
    assert!(b.is_break());
}

#[test]
fn control_flow_into_value() {
    let c = ControlFlow::<i32, i32>::Continue(0);
    assert_eq!(c.into_value(), 0);

    let b = ControlFlow::<i32, i32>::Break(1);
    assert_eq!(b.into_value(), 1);
}

#[test]
fn control_flow_continue_value() {
    let c = ControlFlow::<i32, i32>::Continue(0);
    assert_eq!(c.continue_value(), Some(0));

    let b = ControlFlow::<i32, i32>::Break(1);
    assert_eq!(b.continue_value(), None);
}

#[test]
fn bound_methods() {
    let one = Bound::Included(&0).copied().map(|x| x + 1);
    assert_eq!(one, Bound::Included(1));

    let two = Bound::Excluded(&1).copied().map(|x| x + 1);
    assert_eq!(two, Bound::Excluded(2));

    let unbounded = Bound::<&i32>::Unbounded.copied().map(|x| x + 1);
    assert_eq!(unbounded, Bound::Unbounded);
}

#[test]
fn range_bound_map() {
    use core::ops::Bound;

    use Bound::*;

    let bound_string = Included("Hello, World!");
    assert_eq!(bound_string.map(|s| s.len()), Included(13));

    let bound_string = Excluded("Hello, World!");
    assert_eq!(bound_string.map(|s| s.len()), Excluded(13));

    let unbounded_string: Bound<String> = Unbounded;
    assert_eq!(unbounded_string.map(|s| s.len()), Unbounded);
}

#[test]
fn wrapped_call_once() {
    core::ferrocene_test::test_wrapped_call_once();
}

// Covers: <core::ops::range::RangeInclusive<A> as core::iter::range::RangeInclusiveIteratorImpl>::{spec_next,spec_next_back,spec_try_fold,spec_try_rfold}
#[test]
fn test_range_inclusive_iterator_impl() {
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    struct UntrustedStep(i32);

    impl core::iter::Step for UntrustedStep {
        fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
            i32::steps_between(&start.0, &end.0)
        }

        fn forward_checked(start: Self, count: usize) -> Option<Self> {
            i32::forward_checked(start.0, count).map(|a| UntrustedStep(a))
        }

        fn backward_checked(start: Self, count: usize) -> Option<Self> {
            i32::backward_checked(start.0, count).map(|a| UntrustedStep(a))
        }
    }

    let mut ri = core::ops::RangeInclusive::new(UntrustedStep(0), UntrustedStep(5));

    assert_eq!(Some(UntrustedStep(0)), ri.next());
    assert_eq!(Some(UntrustedStep(5)), ri.next_back());

    let mut ri2 = ri.clone();
    assert_eq!(Some(10), ri2.try_fold(0, |acc, x| Some(acc + x.0)));

    let mut ri3 = ri.clone();
    assert_eq!(Some(10), ri3.try_rfold(0, |acc, x| Some(acc + x.0)));
}

// Covers `core::iter::range::<impl core::iter::traits::iterator::Iterator for core::ops::range::RangeInclusive<A>>::count`
#[test]
fn range_inclusve_count_empty() {
    assert_eq!((3..=2).count(), 0);
}
