mod atomic;
mod borrow;
mod iter;
mod num;
mod time;

use core::cmp::Ordering;
use core::mem::MaybeUninit;
use core::ops::{Bound, ControlFlow};
use core::panic::Location;
use core::sync::atomic::AtomicU32;
use core::time::Duration;

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

#[test]
fn default_chaining_impl() {
    assert!((1, 2) <= (1, 2));
    assert!((3, 4) >= (1, 2));
    assert!((1, 2) < (3, 4));
    assert!((3, 4) > (1, 2));

    assert!((1, 2) <= (1, 2));
    assert!((1, 2) >= (1, 2));
    assert_eq!((1, 2) <= (2, 2), true);
    assert_eq!((2, 2) >= (1, 2), true);
}

#[test]
fn tuple_comparison() {
    let data = [
        ("core::iter::adapters::Chain", 123_usize),
        ("core::iter::adapters::Clone", 456_usize),
        ("core::iter::adapters::Copie", 789_usize),
        ("core::iter::adapters::Cycle", 123_usize),
        ("core::iter::adapters::Flatt", 456_usize),
        ("core::iter::adapters::TakeN", 789_usize),
    ];

    for val in data.windows(2) {
        let x = val[0];
        let y = val[1];
        assert_eq!([x < y, x <= y, x > y, x >= y], [true, true, false, false]);
    }

    assert!(("1", "2", "3") < ("1", "2", "4"));
    assert!(("1", "2", "3") < ("1", "2", "4"));
    #[derive(PartialOrd, PartialEq)]
    struct Float(f32);
    assert!(!((Float(f32::NAN), Float(f32::NAN), "3") < (Float(1.0), Float(f32::NAN), "4")));
}

macro_rules! nums_to_bytes_tests {
    ($($int:ty),*) => {
        $({
            let int = <$int>::MAX;
            let le_bytes = int.to_le_bytes();
            let be_bytes = int.to_be_bytes();

            assert_eq!(<$int>::from_le_bytes(le_bytes), int);
            assert_eq!(<$int>::from_be_bytes(be_bytes), int);
        })*
    }
}

#[test]
fn numbers_to_bytes() {
    nums_to_bytes_tests! {
        u8, u16, u32, u64, u128, usize,
        i8, i16, i32, i64, i128, isize,
        f32, f64
    }
}

#[test]
fn abs_diff_vectorization() {
    fn sad_iter(a: &[u8; 8], b: &[u8; 8]) -> u32 {
        a.iter().zip(b).map(|(&a, &b)| a.abs_diff(b) as u32).sum()
    }

    fn sad_loop(a: &[u8; 8], b: &[u8; 8]) -> u32 {
        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i].abs_diff(b[i]) as u32;
        }
        sum
    }

    let max_buf = [u8::MAX; 8];
    let min_buf = [u8::MIN; 8];

    assert_eq!(sad_iter(&max_buf, &min_buf), 8 * (u8::MAX as u32));
    assert_eq!(sad_loop(&max_buf, &min_buf), 8 * (u8::MAX as u32));
}

#[test]
fn maybe_uninit() {
    let mut maybe = MaybeUninit::new(u64::MIN);

    let mut ptr = MaybeUninit::slice_as_ptr(maybe.as_bytes());

    for _ in 0..core::mem::size_of::<u64>() {
        assert_eq!(unsafe { ptr.read() }, u8::MIN);
        ptr = unsafe { ptr.add(1) };
    }

    for byte in maybe.as_bytes_mut() {
        byte.write(u8::MAX);
    }

    assert_eq!(*unsafe { maybe.assume_init_ref() }, u64::MAX);
}

#[test]
fn slice_methods() {
    let mut arr = [0; 10];
    let slice = arr.as_mut_slice();

    assert_eq!(slice.len(), 10);

    assert!(slice.first_chunk_mut::<11>().is_none());
    assert!(slice.first_chunk_mut::<1>().is_some());

    assert!(slice.split_first_mut().is_some());

    let mut empty = [0; 0];
    assert!(empty.as_mut_slice().split_first_mut().is_none());
}

#[test]
fn str_methods() {
    let s = <&str>::default();
    assert_eq!(s.as_str(), "");

    let mut buf = String::from("a");
    let s = unsafe { core::slice::from_raw_parts_mut(buf.as_mut_str().as_mut_ptr(), 1) };
    s[0] = b'b';
    assert_eq!(buf, "b");
}

#[test]
fn duration_methods() {
    let secs = Duration::new(1, 0);

    assert_eq!(secs.as_micros(), 1_000_000);

    assert_eq!(secs.as_millis(), 1_000);
    assert_eq!(secs.as_millis_f32(), 1_000.0);
    assert_eq!(secs.as_millis_f64(), 1_000.0);

    assert_eq!(secs.as_secs_f32(), 1.0);

    assert!(!secs.is_zero());
    assert!(Duration::from_secs(0).is_zero());

    assert_eq!(Duration::from_hours(1).as_secs(), 60 * 60);
    assert_eq!(Duration::from_days(1).as_secs(), 60 * 60 * 24);
    assert_eq!(Duration::from_weeks(1).as_secs(), 60 * 60 * 24 * 7);
}

#[test]
fn unit_comparisons() {
    assert!(!(() != ()));
    assert!(!(&() != &mut ()));
    assert!(!(&mut () != &()));
    assert!(!(() < ()));
}

#[test]
fn type_name() {
    assert_eq!(core::any::type_name::<u8>(), core::any::type_name_of_val(&0u8));
}

#[test]
fn assume() {
    unsafe { core::hint::assert_unchecked(true) };
}

#[test]
fn array_iter_mut() {
    let mut arr = [0u8; 10];

    for x in &mut arr {
        *x = 1;
    }

    let sum = arr.into_iter().sum::<u8>();

    assert_eq!(sum as usize, arr.len());
}

#[test]
fn slice_partial_eq() {
    #[derive(Debug, Eq, PartialEq, Copy, Clone)]
    struct Byte(u8);

    let a1 = [Byte(0u8); 100];
    let a2 = [Byte(0u8); 99];
    let mut a3 = [Byte(0u8); 100];

    assert_ne!(a1.as_slice(), a2.as_slice());

    assert_eq!(a1.as_slice(), a3.as_slice());

    a3[a3.len() - 1] = Byte(1);
    assert_ne!(a1.as_slice(), a3.as_slice());
}

#[test]
fn as_mut_array() {
    let mut arr = [0u8; 1];
    let slice = arr.as_mut_slice();

    assert!(slice.as_mut_array::<2>().is_none());
    assert!(slice.as_mut_array::<1>().is_some());
}

#[test]
fn exact_iterator_mut_ref_is_empty() {
    assert!((&mut (0..0)).is_empty());
    assert!(!(&mut (0..1)).is_empty());
}

#[test]
fn chunks_exact_is_empty() {
    assert!([0; 10].chunks_exact(11).is_empty());
    assert!(![0; 10].chunks_exact(2).is_empty());

    assert!([0; 10].chunks_exact_mut(11).is_empty());
    assert!(![0; 10].chunks_exact_mut(2).is_empty());
}
macro_rules! ilog2_loop {
    ($(($T:ty, $ilog2_max:expr) => $fn:ident,)*) => {
        $(
            #[test]
            fn $fn() {
                assert_eq!(<$T>::MAX.ilog2(), $ilog2_max);
                for i in 0..=$ilog2_max {
                    let p = (2 as $T).pow(i as u32);
                    if p >= 2 {
                        assert_eq!((p - 1).ilog2(), i - 1);
                    }
                    assert_eq!(p.ilog2(), i);
                    if p >= 2 {
                        assert_eq!((p + 1).ilog2(), i);
                    }

                    // also check `x.ilog(2)`
                    if p >= 2 {
                        assert_eq!((p - 1).ilog(2), i - 1);
                    }
                    assert_eq!(p.ilog(2), i);
                    if p >= 2 {
                        assert_eq!((p + 1).ilog(2), i);
                    }
                }
            }
        )*
    };
}

ilog2_loop! {
    (u8, 7) => ilog2_u8,
    (u16, 15) => ilog2_u16,
    (u32, 31) => ilog2_u32,
    (u64, 63) => ilog2_u64,
    (u128, 127) => ilog2_u128,
    (i8, 6) => ilog2_i8,
    (i16, 14) => ilog2_i16,
    (i32, 30) => ilog2_i32,
    (i64, 62) => ilog2_i64,
    (i128, 126) => ilog2_i128,
}

macro_rules! nonpositive_ilog2 {
    ($($T:ty => $fn:ident,)*) => {
        $(
            #[test]
            #[should_panic]
            fn $fn() {
                let _ = (-1 as $T).ilog2();
            }
        )*
    };
}

nonpositive_ilog2! {
    i8 => nonpositive_ilog2_of_i8,
    i16 => nonpositive_ilog2_of_i16,
    i32 => nonpositive_ilog2_of_i32,
    i64 => nonpositive_ilog2_of_i64,
    i128 => nonpositive_ilog2_of_i128,
}

#[test]
fn str_bytes() {
    let s = "yellow submarine";

    assert!(s.bytes().all(|b| b.is_ascii()));
    assert!(s.bytes().any(|b| b.is_ascii_whitespace()));
    assert!(s.bytes().find(|b| *b == b'i').is_some());
    assert_eq!(s.bytes().position(|b| b == b's'), Some(7));
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

// covers:
// - `<core::mem::Discriminant<T> as core::cmp::PartialEq>::eq`
// - `core::mem::discriminant`
#[test]
fn discriminant() {
    enum Foo {
        A,
        B,
    }
    assert_eq!(core::mem::discriminant(&Foo::A), core::mem::discriminant(&Foo::A));
    assert_ne!(core::mem::discriminant(&Foo::A), core::mem::discriminant(&Foo::B));
}

// covers:
// - `<core::mem::maybe_uninit::MaybeUninit<T> as core::clone::Clone>::clone`
// - `core::mem::maybe_uninit::MaybeUninit::<T>::as_bytes`
// - `core::mem::maybe_uninit::MaybeUninit::<T>::as_bytes_mut`
#[test]
fn maybe_uninit() {
    let mut source = core::mem::MaybeUninit::<u64>::uninit();
    // This looks rather artificial but it guarantees that:
    // - `Clone` implementation for `MaybeUninit` is covered.
    // - `destination` is uninitialized.
    let mut destination = source.clone();

    // Initialize `source`
    source.write(u64::MAX);

    // Initialize `destination` by copying each byte of `source` into `destination`.
    for (src, dst) in source.as_bytes().into_iter().zip(destination.as_bytes_mut()) {
        let val = unsafe { src.assume_init_read() };
        dst.write(val);
    }

    // SAFETY: This was initialized to `u64::MAX`
    let source = unsafe { source.assume_init() };
    // SAFETY: This was initialized by copying the initialized bytes of `source` into it.
    let destination = unsafe { destination.assume_init() };

    assert_eq!(source, u64::MAX);
    assert_eq!(destination, u64::MAX);
}

// covers `core::ptr::const_ptr::<impl *const T>::align_offset`
#[test]
#[should_panic = "align_offset: align is not a power-of-two"]
fn non_power_of_two_align_offset() {
    let ptr: *const () = &();
    let _ = ptr.align_offset(3);
}

// covers:
// - `core::ptr::non_null::NonNull::<T>::sub`
// - `core::ptr::mut_ptr::<impl *mut T>::sub`
#[test]
fn zst_sub_is_noop() {
    let ptr = core::ptr::NonNull::from_ref(&());
    // SAFETY: `ptr` is a pointer to a ZST so substracting anything from it is a noop.
    assert_eq!(ptr, unsafe { ptr.sub(isize::MAX as usize) });
    assert_eq!(ptr.as_ptr(), unsafe { ptr.as_ptr().sub(isize::MAX as usize) });
}

// covers:
// - `core::ptr::const_ptr::<impl core::cmp::Ord for *const T>::cmp`
// - `core::ptr::const_ptr::<impl core::cmp::PartialOrd for *const T>::ge`
// - `core::ptr::const_ptr::<impl core::cmp::PartialOrd for *const T>::gt`
// - `core::ptr::const_ptr::<impl core::cmp::PartialOrd for *const T>::le`
// - `core::ptr::const_ptr::<impl core::cmp::PartialOrd for *const T>::lt`
// - `core::ptr::const_ptr::<impl core::cmp::PartialOrd for *const T>::partial_cmp`
#[test]
fn ptr_partial_ord() {
    let arr = [0, 1];

    let fst = arr.as_ptr();
    let snd = unsafe { fst.add(1) };

    assert!(fst.le(&fst));
    assert!(fst.le(&snd));
    assert!(fst.lt(&snd));

    assert!(snd.ge(&snd));
    assert!(snd.ge(&fst));
    assert!(snd.gt(&fst));

    assert_eq!(fst.partial_cmp(&fst), Some(core::cmp::Ordering::Equal));
    assert_eq!(fst.partial_cmp(&snd), Some(core::cmp::Ordering::Less));
    assert_eq!(snd.partial_cmp(&fst), Some(core::cmp::Ordering::Greater));
}

// covers:
// - `core::ptr::read_volatile`
// - `core::ptr::write_volatile`
#[test]
fn volatile_ops() {
    let mut x = 0;
    let y = &mut x as *mut i32;

    unsafe {
        std::ptr::write_volatile(y, 12);
        assert_eq!(std::ptr::read_volatile(y), 12);
    }
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
