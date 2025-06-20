mod control_flow;
mod from_residual;
mod function;

use core::ops::{
    Bound, Deref, DerefMut, IntoBounds, OneSidedRange, Range, RangeBounds, RangeFrom, RangeFull,
    RangeInclusive, RangeTo, RangeToInclusive,
};

// Test the Range structs and syntax.

#[test]
fn test_range() {
    let r = Range { start: 2, end: 10 };
    let mut count = 0;
    for (i, ri) in r.enumerate() {
        assert_eq!(ri, i + 2);
        assert!(ri >= 2 && ri < 10);
        count += 1;
    }
    assert_eq!(count, 8);
}

#[test]
fn test_range_from() {
    let r = RangeFrom { start: 2 };
    let mut count = 0;
    for (i, ri) in r.take(10).enumerate() {
        assert_eq!(ri, i + 2);
        assert!(ri >= 2 && ri < 12);
        count += 1;
    }
    assert_eq!(count, 10);
}

#[test]
fn test_range_to() {
    // Not much to test.
    let _ = RangeTo { end: 42 };
}

#[test]
fn test_full_range() {
    // Not much to test.
    let _ = RangeFull;
}

#[test]
fn test_range_inclusive() {
    let mut r = RangeInclusive::new(1i8, 2);
    assert_eq!(r.start(), &1);
    assert_eq!(r.next(), Some(1));
    assert_eq!(r.next(), Some(2));
    assert_eq!(r.next(), None);
    assert_eq!(r.start(), &2);

    r = RangeInclusive::new(127i8, 127);
    assert_eq!(r.start(), &127);
    assert_eq!(r.next(), Some(127));
    assert_eq!(r.next(), None);
    assert_eq!(r.start(), &127);

    r = RangeInclusive::new(-128i8, -128);
    assert_eq!(r.start(), &-128);
    assert_eq!(r.next_back(), Some(-128));
    assert_eq!(r.next_back(), None);
    assert_eq!(r.start(), &-128);

    // degenerate
    r = RangeInclusive::new(1, -1);
    assert_eq!(r.start(), &1);
    assert_eq!(r.size_hint(), (0, Some(0)));
    assert_eq!(r.next(), None);
    assert_eq!(r.start(), &1);
}

#[test]
fn test_range_to_inclusive() {
    // Not much to test.
    let _ = RangeToInclusive { end: 42 };
}

#[test]
fn test_range_is_empty() {
    assert!(!(0.0..10.0).is_empty());
    assert!((-0.0..0.0).is_empty());
    assert!((10.0..0.0).is_empty());

    assert!(!(f32::NEG_INFINITY..f32::INFINITY).is_empty());
    assert!((f32::EPSILON..f32::NAN).is_empty());
    assert!((f32::NAN..f32::EPSILON).is_empty());
    assert!((f32::NAN..f32::NAN).is_empty());

    assert!(!(0.0..=10.0).is_empty());
    assert!(!(-0.0..=0.0).is_empty());
    assert!((10.0..=0.0).is_empty());

    assert!(!(f32::NEG_INFINITY..=f32::INFINITY).is_empty());
    assert!((f32::EPSILON..=f32::NAN).is_empty());
    assert!((f32::NAN..=f32::EPSILON).is_empty());
    assert!((f32::NAN..=f32::NAN).is_empty());
}

#[test]
fn test_bound_cloned_unbounded() {
    assert_eq!(Bound::<&u32>::Unbounded.cloned(), Bound::Unbounded);
}

#[test]
fn test_bound_cloned_included() {
    assert_eq!(Bound::Included(&3).cloned(), Bound::Included(3));
}

#[test]
fn test_bound_cloned_excluded() {
    assert_eq!(Bound::Excluded(&3).cloned(), Bound::Excluded(3));
}

#[test]
fn test_bound_as_ref() {
    assert_eq!((&Bound::Included(3)).as_ref(), Bound::Included(&3));
    assert_eq!((&Bound::Excluded(3)).as_ref(), Bound::Excluded(&3));
    assert_eq!((&Bound::<&u32>::Unbounded).as_ref(), Bound::Unbounded);
}

#[test]
fn test_bound_as_mut() {
    assert_eq!((&mut Bound::Included(3)).as_mut(), Bound::Included(&mut 3));
    assert_eq!((&mut Bound::Excluded(3)).as_mut(), Bound::Excluded(&mut 3));
    assert_eq!((&mut Bound::<&u32>::Unbounded).as_mut(), Bound::Unbounded);
}

#[test]
fn test_bound_map() {
    assert_eq!((&mut Bound::Included(3)).map(|x| x + 1), Bound::Included(4));
    assert_eq!((&mut Bound::Excluded(3)).map(|x| x + 1), Bound::Excluded(4));
    assert_eq!((&mut Bound::<&u32>::Unbounded).map(|x| x + 1), Bound::Unbounded);
}

#[test]
#[allow(unused_comparisons)]
#[allow(unused_mut)]
fn test_range_syntax() {
    let mut count = 0;
    for i in 0_usize..10 {
        assert!(i >= 0 && i < 10);
        count += i;
    }
    assert_eq!(count, 45);

    let mut count = 0;
    let mut range = 0_usize..10;
    for i in range {
        assert!(i >= 0 && i < 10);
        count += i;
    }
    assert_eq!(count, 45);

    let mut count = 0;
    let mut rf = 3_usize..;
    for i in rf.take(10) {
        assert!(i >= 3 && i < 13);
        count += i;
    }
    assert_eq!(count, 75);

    let _ = 0_usize..4 + 4 - 3;

    fn foo() -> isize {
        42
    }
    let _ = 0..foo();

    let _ = { &42..&100 }; // references to literals are OK
    let _ = ..42_usize;

    // Test we can use two different types with a common supertype.
    let x = &42;
    {
        let y = 42;
        let _ = x..&y;
    }
}

#[test]
#[allow(dead_code)]
fn test_range_syntax_in_return_statement() {
    fn return_range_to() -> RangeTo<i32> {
        return ..1;
    }
    fn return_full_range() -> RangeFull {
        return ..;
    }
    // Not much to test.
}

#[test]
fn range_structural_match() {
    // test that all range types can be structurally matched upon

    const RANGE: Range<usize> = 0..1000;
    match RANGE {
        RANGE => {}
        _ => unreachable!(),
    }

    const RANGE_FROM: RangeFrom<usize> = 0..;
    match RANGE_FROM {
        RANGE_FROM => {}
        _ => unreachable!(),
    }

    const RANGE_FULL: RangeFull = ..;
    match RANGE_FULL {
        RANGE_FULL => {}
    }

    const RANGE_INCLUSIVE: RangeInclusive<usize> = 0..=999;
    match RANGE_INCLUSIVE {
        RANGE_INCLUSIVE => {}
        _ => unreachable!(),
    }

    const RANGE_TO: RangeTo<usize> = ..1000;
    match RANGE_TO {
        RANGE_TO => {}
        _ => unreachable!(),
    }

    const RANGE_TO_INCLUSIVE: RangeToInclusive<usize> = ..=999;
    match RANGE_TO_INCLUSIVE {
        RANGE_TO_INCLUSIVE => {}
        _ => unreachable!(),
    }
}

#[test]
fn test_range_bounds() {
    let r = Range { start: 2, end: 42 };
    assert_eq!(r.start_bound(), Bound::Included(&2));
    assert_eq!(r.end_bound(), Bound::Excluded(&42));
    assert_eq!(r.into_bounds(), (Bound::Included(2), Bound::Excluded(42)));

    let r = Range { start: &2, end: &42 };
    assert_eq!(r.start_bound(), Bound::Included(&&2));
    assert_eq!(r.end_bound(), Bound::Excluded(&&42));
    assert_eq!(r.into_bounds(), (Bound::Included(&2), Bound::Excluded(&42)));

    let r = RangeFrom { start: 2 };
    assert_eq!(r.start_bound(), Bound::Included(&2));
    assert_eq!(r.end_bound(), Bound::Unbounded);
    assert_eq!(r.into_bounds(), (Bound::Included(2), Bound::Unbounded));

    let r = RangeFrom { start: &2 };
    assert_eq!(r.start_bound(), Bound::Included(&&2));
    assert_eq!(r.end_bound(), Bound::<&i32>::Unbounded);
    assert_eq!(r.into_bounds(), (Bound::Included(&2), Bound::Unbounded));

    let r = RangeTo { end: 42 };
    assert_eq!(r.start_bound(), Bound::Unbounded);
    assert_eq!(r.end_bound(), Bound::Excluded(&42));
    assert_eq!(r.into_bounds(), (Bound::Unbounded, Bound::Excluded(42)));

    let r = RangeTo { end: &42 };
    assert_eq!(r.start_bound(), Bound::<&i32>::Unbounded);
    assert_eq!(r.end_bound(), Bound::Excluded(&&42));
    assert_eq!(r.into_bounds(), (Bound::Unbounded, Bound::Excluded(&42)));

    let r = RangeFull;
    assert_eq!(r.start_bound(), Bound::<&i32>::Unbounded);
    assert_eq!(r.end_bound(), Bound::<&i32>::Unbounded);
    assert_eq!(r.into_bounds(), (Bound::<i32>::Unbounded, Bound::<i32>::Unbounded));

    let mut r = RangeInclusive::new(2, 42);
    assert_eq!(r.start_bound(), Bound::Included(&2));
    assert_eq!(r.end_bound(), Bound::Included(&42));
    assert_eq!(r.clone().into_bounds(), (Bound::Included(2), Bound::Included(42)));
    while r.next().is_some() {}
    assert_eq!(r.end_bound(), Bound::Excluded(&42));

    let r = RangeInclusive::new(&2, &42);
    assert_eq!(r.start_bound(), Bound::Included(&&2));
    assert_eq!(r.end_bound(), Bound::Included(&&42));
    assert_eq!(r.clone().into_bounds(), (Bound::Included(&2), Bound::Included(&42)));

    let r = RangeToInclusive { end: 42 };
    assert_eq!(r.start_bound(), Bound::Unbounded);
    assert_eq!(r.end_bound(), Bound::Included(&42));
    assert_eq!(r.into_bounds(), (Bound::Unbounded, Bound::Included(42)));

    let r = RangeToInclusive { end: &42 };
    assert_eq!(r.start_bound(), Bound::<&i32>::Unbounded);
    assert_eq!(r.end_bound(), Bound::Included(&&42));
    assert_eq!(r.into_bounds(), (Bound::Unbounded, Bound::Included(&42)));
}

#[test]
fn test_range_bounds_tuple() {
    let r = (Bound::Unbounded, Bound::Included(42));
    assert_eq!(r.start_bound(), Bound::Unbounded);
    assert_eq!(r.end_bound(), Bound::Included(&42));
    assert_eq!(r.into_bounds(), (Bound::Unbounded, Bound::Included(42)));

    let r = (Bound::Unbounded, Bound::Excluded(42));
    assert_eq!(r.start_bound(), Bound::Unbounded);
    assert_eq!(r.end_bound(), Bound::Excluded(&42));
    assert_eq!(r.into_bounds(), (Bound::Unbounded, Bound::Excluded(42)));

    let r = (Bound::<i32>::Unbounded, Bound::Unbounded);
    assert_eq!(r.start_bound(), Bound::Unbounded);
    assert_eq!(r.end_bound(), Bound::Unbounded);
    assert_eq!(r.into_bounds(), (Bound::Unbounded, Bound::Unbounded));

    let r = (Bound::Included(2), Bound::Included(42));
    assert_eq!(r.start_bound(), Bound::Included(&2));
    assert_eq!(r.end_bound(), Bound::Included(&42));
    assert_eq!(r.into_bounds(), (Bound::Included(2), Bound::Included(42)));

    let r = (Bound::Included(2), Bound::Excluded(42));
    assert_eq!(r.start_bound(), Bound::Included(&2));
    assert_eq!(r.end_bound(), Bound::Excluded(&42));
    assert_eq!(r.into_bounds(), (Bound::Included(2), Bound::Excluded(42)));

    let r = (Bound::Included(2), Bound::Unbounded);
    assert_eq!(r.start_bound(), Bound::Included(&2));
    assert_eq!(r.end_bound(), Bound::Unbounded);
    assert_eq!(r.into_bounds(), (Bound::Included(2), Bound::Unbounded));

    let r = (Bound::Excluded(2), Bound::Included(42));
    assert_eq!(r.start_bound(), Bound::Excluded(&2));
    assert_eq!(r.end_bound(), Bound::Included(&42));
    assert_eq!(r.into_bounds(), (Bound::Excluded(2), Bound::Included(42)));

    let r = (Bound::Excluded(2), Bound::Excluded(42));
    assert_eq!(r.start_bound(), Bound::Excluded(&2));
    assert_eq!(r.end_bound(), Bound::Excluded(&42));
    assert_eq!(r.into_bounds(), (Bound::Excluded(2), Bound::Excluded(42)));

    let r = (Bound::Excluded(2), Bound::Unbounded);
    assert_eq!(r.start_bound(), Bound::Excluded(&2));
    assert_eq!(r.end_bound(), Bound::Unbounded);
    assert_eq!(r.into_bounds(), (Bound::Excluded(2), Bound::Unbounded));
}

#[test]
fn test_one_sided_range() {
    RangeFrom { start: 2 }.bound();
    RangeTo { end: 42 }.bound();
    RangeToInclusive { end: 42 }.bound();
}

// Test Deref implementations

#[test]
fn deref_mut_on_ref() {
    // Test that `&mut T` implements `DerefMut<T>`

    fn inc<T: Deref<Target = isize> + DerefMut>(mut t: T) {
        *t += 1;
    }

    let mut x: isize = 5;
    inc(&mut x);
    assert_eq!(x, 6);
}

#[test]
fn deref_on_ref() {
    // Test that `&T` and `&mut T` implement `Deref<T>`

    fn deref<U: Copy, T: Deref<Target = U>>(t: T) -> U {
        *t
    }

    let x: isize = 3;
    let y = deref(&x);
    assert_eq!(y, 3);

    let mut x: isize = 4;
    let y = deref(&mut x);
    assert_eq!(y, 4);
}

#[test]
#[allow(unreachable_code)]
fn test_not_never() {
    if !return () {}
}
