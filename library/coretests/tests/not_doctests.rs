use core::cmp::Ordering;
use core::ops::{Bound, ControlFlow};
use core::panic::Location;
use core::sync::atomic::AtomicU32;

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

#[test]
#[should_panic = "reached"]
fn filter_option() {
    let _ = Some("foo").filter(|_| panic!("reached"));
}

#[test]
#[should_panic = "reached"]
fn inspect_option() {
    let _ = Some("foo").inspect(|_| panic!("reached"));
}

#[test]
fn result_methods() {
    let s = String::from("hello");
    let mut ok = Ok::<&&String, &&String>(&&s);
    let mut err = Err::<&&String, &&String>(&&s);

    let mut len = None;
    err = err.inspect(|s| {
        len = Some(s.len());
    });
    assert!(len.is_none());
    assert!(err.is_err_and(|&x| x == &s));
    assert!(!err.is_ok_and(|&x| x == &s));

    ok = ok.inspect(|s| {
        len = Some(s.len());
    });
    assert_eq!(len, Some(5));
    assert!(ok.is_ok_and(|&x| x == &s));
    assert!(!ok.is_err_and(|&x| x == &s));

    let mut len = None;
    ok = ok.inspect_err(|s| {
        len = Some(s.len());
    });
    assert!(len.is_none());
    assert!(ok.is_ok_and(|&x| x == &s));
    assert!(!ok.is_err_and(|&x| x == &s));

    err = err.inspect_err(|s| {
        len = Some(s.len());
    });
    assert_eq!(len, Some(5));
    assert!(err.is_err_and(|&x| x == &s));
    assert!(!err.is_ok_and(|&x| x == &s));

    assert_eq!(ok.copied().cloned().map_or_default(|s| s.len()), 5);
    assert_eq!(err.copied().cloned().map_or_default(|s| s.len()), 0);

    assert_eq!(ok.copied().cloned().map_or_else(|_| 0, |s| s.len()), 5);
    assert_eq!(err.copied().cloned().map_or_else(|_| 0, |s| s.len()), 0);

    let mut x = 5i32;

    assert!(Ok::<&mut i32, i32>(&mut x).cloned().is_ok_and(|x| x == 5));
    assert!(Ok::<&mut i32, i32>(&mut x).copied().is_ok_and(|x| x == 5));

    assert!(Err::<&mut i32, i32>(x).cloned().is_err_and(|x| x == 5));
    assert!(Err::<&mut i32, i32>(x).copied().is_err_and(|x| x == 5));
}

#[test]
#[should_panic = "reached"]
fn inspect_result() {
    let _ = Ok::<&str, &str>("foo").inspect(|_| panic!("reached"));
}

#[test]
#[should_panic = "reached"]
fn inspect_result_err() {
    let _ = Err::<&str, &str>("foo").inspect_err(|_| panic!("reached"));
}

#[test]
fn atomic_methods() {
    use core::sync::atomic::Ordering::*;
    let atomic = AtomicU32::new(0);

    assert_eq!(unsafe { *atomic.as_ptr() }, 0);

    assert!(atomic.try_update(Relaxed, Relaxed, |_| None).is_err());
    assert!(atomic.try_update(Relaxed, Relaxed, |x| Some(x + 2)).is_ok());

    let mut is_retry = false;
    let result = atomic.try_update(Relaxed, Relaxed, |x| {
        if is_retry {
            Some(x + 1)
        } else {
            is_retry = true;
            atomic.store(4, Relaxed);
            Some(u32::MAX)
        }
    });

    assert!(result.is_ok_and(|x| x == 4));
    assert_eq!(atomic.load(Relaxed), 5);

    assert_eq!(atomic.update(Relaxed, Relaxed, |x| x), 5);
    assert_eq!(atomic.update(Relaxed, Relaxed, |x| x - 1), 5);

    let mut is_retry = false;
    let result = atomic.update(Relaxed, Relaxed, |x| {
        if is_retry {
            x + 1
        } else {
            is_retry = true;
            atomic.store(7, Relaxed);
            u32::MAX
        }
    });

    assert_eq!(result, 7);
    assert_eq!(atomic.into_inner(), 8);

    let mut arr = [0, 1, 2];

    let _ = AtomicU32::from_mut(arr.get_mut(0).unwrap()).get_mut();
    let _ = AtomicU32::get_mut_slice(AtomicU32::from_mut_slice(arr.get_mut(0..1).unwrap()));

    let _ = unsafe { AtomicU32::from_ptr(arr.get_mut(0).unwrap() as *mut u32) };
}

#[test]
fn panic_location() {
    let loc = Location::caller();

    let _ = loc.line();
    let _ = loc.column();
}

#[test]
fn try_cast_aligned() {
    let x = 0u64;

    let aligned: *const u64 = &x;
    let unaligned = unsafe { aligned.byte_add(1) };

    assert!(aligned.try_cast_aligned::<u32>().is_some());
    assert!(unaligned.try_cast_aligned::<u32>().is_none());
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
