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
