use core::cmp::Ordering;
use core::ops::{Bound, ControlFlow};

#[test]
fn ordering_equality() {
    let os = [Ordering::Less, Ordering::Equal, Ordering::Greater];
    for ordering in os {
        let is_eq = ordering.is_eq();
        let is_ne = ordering.is_ne();
        if let Ordering::Equal = ordering {
            assert!(is_eq);
            assert!(!is_ne);
        } else {
            assert!(!is_eq);
            assert!(is_ne);
        }
    }
}

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
fn bound_methods() {
    let one = Bound::Included(&0).copied().map(|x| x + 1);
    assert_eq!(one, Bound::Included(1));

    let two = Bound::Excluded(&1).copied().map(|x| x + 1);
    assert_eq!(two, Bound::Excluded(2));

    let unbounded = Bound::<&i32>::Unbounded.copied().map(|x| x + 1);
    assert_eq!(unbounded, Bound::Unbounded);
}

#[test]
fn option_methods() {
    let s = String::from("hello");
    let mut some = Some(&&s);
    let mut none = None::<&&String>;

    let mut len = None;

    none = none.inspect(|s| {
        len = Some(s.len());
    });
    assert!(len.is_none());
    assert!(none.is_none());

    some = some.inspect(|s| {
        len = Some(s.len());
    });
    assert_eq!(len, Some(5));
    assert!(some.is_some_and(|&x| x == &s));

    assert_eq!(some.copied().cloned().map_or_default(|s| s.len()), 5);
    assert_eq!(none.copied().cloned().map_or_default(|s| s.len()), 0);

    assert_eq!(some.reduce(some, |x, _| x), some);
    assert_eq!(some.reduce(none, |x, _| x), some);
    assert_eq!(none.reduce(some, |x, _| x), some);
    assert_eq!(none.reduce(none, |x, _| x), none);

    assert_eq!(some.xor(some), none);
    assert_eq!(some.xor(none), some);
    assert_eq!(none.xor(some), some);
    assert_eq!(none.xor(none), none);

    assert!(some.filter(|s| s.len() == 5).is_some_and(|&x| x == &s));
    assert!(some.filter(|s| s.len() == 4).is_none());
    assert!(none.filter(|_| true).is_none());

    let mut x = 5i32;

    assert!(Some(&mut x).cloned().is_some_and(|x| x == 5));
    assert!(Some(&mut x).copied().is_some_and(|x| x == 5));

    assert!(None::<&mut i32>.cloned().is_none());
    assert!(None::<&mut i32>.copied().is_none());

    assert_eq!(Some(Some(0i32)).flatten(), Some(0));
    assert_eq!(Some(None::<i32>).flatten(), None);
    assert_eq!(None::<Option<i32>>.flatten(), None);
}
