use core::cmp::Ordering;
use core::ops::{Bound, ControlFlow};
use core::panic::Location;
use core::sync::atomic::{self, AtomicU32};
use core::time::Duration;
use core::{cell, ptr, slice};

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
#[cfg_attr(not(feature = "ferrocene_certified_panic"), should_panic = "reached")]
#[cfg_attr(feature = "ferrocene_certified_panic", should_panic)]
fn filter_option() {
    let _ = Some("foo").filter(|_| panic!("reached"));
}

#[test]
#[cfg_attr(not(feature = "ferrocene_certified_panic"), should_panic = "reached")]
#[cfg_attr(feature = "ferrocene_certified_panic", should_panic)]
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
#[cfg_attr(not(feature = "ferrocene_certified_panic"), should_panic = "reached")]
#[cfg_attr(feature = "ferrocene_certified_panic", should_panic)]
fn inspect_result() {
    let _ = Ok::<&str, &str>("foo").inspect(|_| panic!("reached"));
}

#[test]
#[cfg_attr(not(feature = "ferrocene_certified_panic"), should_panic = "reached")]
#[cfg_attr(feature = "ferrocene_certified_panic", should_panic)]
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
#[expect(deprecated)]
fn atomic_int_compare_and_swap() {
    macro_rules! test_atomic_compare_and_swap {
        ($atomic_t:ty) => {{
            let atomic = <$atomic_t>::new(5);

            assert_eq!(atomic.compare_and_swap(5, 10, atomic::Ordering::Relaxed), 5); // success
            assert_eq!(atomic.compare_and_swap(20, 30, atomic::Ordering::Relaxed), 10); // failure
        }};
    }

    test_atomic_compare_and_swap!(atomic::AtomicU8);
    test_atomic_compare_and_swap!(atomic::AtomicU16);
    test_atomic_compare_and_swap!(atomic::AtomicU32);
    test_atomic_compare_and_swap!(atomic::AtomicU64);
    test_atomic_compare_and_swap!(atomic::AtomicUsize);
    test_atomic_compare_and_swap!(atomic::AtomicI8);
    test_atomic_compare_and_swap!(atomic::AtomicI16);
    test_atomic_compare_and_swap!(atomic::AtomicI32);
    test_atomic_compare_and_swap!(atomic::AtomicI64);
    test_atomic_compare_and_swap!(atomic::AtomicIsize);
}

#[test]
#[expect(deprecated)]
fn atomic_bool_compare_and_swap() {
    let atomic = atomic::AtomicBool::new(false);

    assert_eq!(atomic.compare_and_swap(false, true, atomic::Ordering::Relaxed), false); // success
    assert_eq!(atomic.compare_and_swap(false, true, atomic::Ordering::Relaxed), true); // failure
}

#[test]
#[expect(deprecated)]
fn atomic_ptr_compare_and_swap() {
    let mut pointee1 = [1, 2, 3, 4, 5];
    let ptr1 = pointee1.as_mut_ptr();
    let mut pointee2 = [5, 4, 3, 2, 1];
    let ptr2 = pointee2.as_mut_ptr();

    let atomic = atomic::AtomicPtr::new(ptr1);

    assert_eq!(atomic.compare_and_swap(ptr1, ptr2, atomic::Ordering::Relaxed), ptr1); // success
    assert_eq!(atomic.compare_and_swap(ptr1, ptr2, atomic::Ordering::Relaxed), ptr2); // failure
}

/// While this test case is borderline useless it is essentially the same what LLVM does:
/// [`llvm/llvm-project/libcxx/test/std/atomics/atomics.fences/atomic_signal_fence.pass.cpp`](https://github.com/llvm/llvm-project/blob/f8580c915f0b5205ddc3ae5e8286653ddc1d8d68/libcxx/test/std/atomics/atomics.fences/atomic_signal_fence.pass.cpp)
#[test]
fn atomic_compiler_fence() {
    use atomic::Ordering::*;

    atomic::compiler_fence(Acquire);
    atomic::compiler_fence(Release);
    atomic::compiler_fence(AcqRel);
    atomic::compiler_fence(SeqCst);
}

#[test]
#[should_panic]
#[expect(invalid_atomic_ordering)]
fn atomic_compiler_fence_relaxed() {
    atomic::compiler_fence(atomic::Ordering::Relaxed);
}

#[test]
fn refcell_replace_with() {
    let mut x = cell::RefCell::new(5);
    assert_eq!(x.replace_with(|y| *y + 10), 5);
    assert_eq!(*x.get_mut(), 15);
}

#[test]
fn refcell_take() {
    let mut x = cell::RefCell::new(5);
    assert_eq!(x.take(), 5);
    assert_eq!(*x.get_mut(), 0);
}

#[test]
fn str_from_utf8_ok() {
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = str::from_utf8(&sparkle_heart);
    assert_eq!(Ok("💖"), sparkle_heart);
}

#[test]
fn str_from_utf8_err() {
    let sparkle_heart_err = vec![0, 159, 146, 150];
    let sparkle_heart_err = str::from_utf8(&sparkle_heart_err);
    assert!(sparkle_heart_err.is_err());
}

#[test]
fn str_from_utf8_mut_ok() {
    let mut sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = str::from_utf8_mut(&mut sparkle_heart);
    assert_eq!(Ok("💖"), sparkle_heart.as_deref());
}

#[test]
fn str_from_utf8_mut_err() {
    let mut sparkle_heart_err = vec![0, 159, 146, 150];
    let sparkle_heart_err = str::from_utf8_mut(&mut sparkle_heart_err);
    assert!(sparkle_heart_err.is_err());
}

#[test]
fn str_from_utf8_unchecked_ok() {
    let sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = unsafe { str::from_utf8_unchecked(&sparkle_heart) };
    assert_eq!("💖", sparkle_heart);
}

#[test]
#[should_panic]
fn str_from_utf8_unchecked_err() {
    let sparkle_heart_err = vec![0, 159, 146, 150];
    let sparkle_heart_err = unsafe { str::from_utf8_unchecked(&sparkle_heart_err) };
    assert_eq!("💖", sparkle_heart_err);
}

#[test]
fn str_from_utf8_unchecked_mut_ok() {
    let mut sparkle_heart = vec![240, 159, 146, 150];
    let sparkle_heart = unsafe { str::from_utf8_unchecked_mut(&mut sparkle_heart) };
    assert_eq!("💖", sparkle_heart);
}

#[test]
#[should_panic]
fn str_from_utf8_unchecked_mut_err() {
    let mut sparkle_heart_err = vec![0, 159, 146, 150];
    let sparkle_heart_err = unsafe { str::from_utf8_unchecked_mut(&mut sparkle_heart_err) };
    assert_eq!("💖", sparkle_heart_err);
}

#[test]
fn str_utf8_error() {
    let sparkle_heart_err = vec![0, 159, 146, 150];
    let sparkle_heart_err = str::from_utf8(&sparkle_heart_err).unwrap_err();

    assert_eq!(sparkle_heart_err.valid_up_to(), 1);
    assert_eq!(sparkle_heart_err.error_len(), Some(1));
}

const STR_MUT_BYTES: [u8; 13] =
    [b'H', b'e', b'l', b'l', b'o', b',', b' ', b'W', b'o', b'r', b'l', b'd', b'!'];

#[test]
fn str_slice_index_range() {
    use slice::SliceIndex;

    let str_ref = "Hello, World!";
    let str_ptr = str_ref as *const str;

    let mut str_mut_bytes = STR_MUT_BYTES;
    let str_mut: &mut str = str::from_utf8_mut(&mut str_mut_bytes).unwrap();
    let str_mut_ptr = str_mut as *mut str;

    assert_eq!(Some("Hell"), SliceIndex::get(0..4, str_ref));
    assert_eq!(Some("Hell"), SliceIndex::get_mut(0..4, str_mut).as_deref());

    assert_eq!(None, SliceIndex::get(100..100, str_ref));
    assert_eq!(None, SliceIndex::get_mut(100..100, str_mut));

    assert_eq!("Hell", SliceIndex::index(0..4, str_ref));
    assert_eq!("Hell", SliceIndex::index_mut(0..4, str_mut));

    assert!(ptr::addr_eq(str_ptr, unsafe { SliceIndex::get_unchecked(0..4, str_ptr) }));
    assert!(ptr::addr_eq(str_mut_ptr, unsafe { SliceIndex::get_unchecked(0..4, str_mut_ptr) }));
}

#[test]
fn str_slice_index_range_from() {
    use slice::SliceIndex;

    let str_ref = "Hello, World!";
    let str_ptr = str_ref as *const str;

    let mut str_mut_bytes = STR_MUT_BYTES;
    let str_mut: &mut str = str::from_utf8_mut(&mut str_mut_bytes).unwrap();
    let str_mut_ptr = str_mut as *mut str;

    assert_eq!(Some("Hello, World!"), SliceIndex::get(0.., str_ref));
    assert_eq!(Some("Hello, World!"), SliceIndex::get_mut(0.., str_mut).as_deref());

    assert_eq!(None, SliceIndex::get(100.., str_ref));
    assert_eq!(None, SliceIndex::get_mut(100.., str_mut));

    assert_eq!("Hello, World!", SliceIndex::index(0.., str_ref));
    assert_eq!("Hello, World!", SliceIndex::index_mut(0.., str_mut));

    assert!(ptr::addr_eq(str_ptr, unsafe { SliceIndex::get_unchecked(0.., str_ptr) }));
    assert!(ptr::addr_eq(str_mut_ptr, unsafe { SliceIndex::get_unchecked(0.., str_mut_ptr) }));
}

#[test]
fn str_slice_index_range_to() {
    use slice::SliceIndex;

    let str_ref = "Hello, World!";
    let str_ptr = str_ref as *const str;

    let mut str_mut_bytes = STR_MUT_BYTES;
    let str_mut: &mut str = str::from_utf8_mut(&mut str_mut_bytes).unwrap();
    let str_mut_ptr = str_mut as *mut str;

    assert_eq!(Some("Hell"), SliceIndex::get(..4, str_ref));
    assert_eq!(Some("Hell"), SliceIndex::get_mut(..4, str_mut).as_deref());

    assert_eq!(None, SliceIndex::get(..100, str_ref));
    assert_eq!(None, SliceIndex::get_mut(..100, str_mut));

    assert_eq!("Hell", SliceIndex::index(..4, str_ref));
    assert_eq!("Hell", SliceIndex::index_mut(..4, str_mut));

    assert!(ptr::addr_eq(str_ptr, unsafe { SliceIndex::get_unchecked(..4, str_ptr) }));
    assert!(ptr::addr_eq(str_mut_ptr, unsafe { SliceIndex::get_unchecked(..4, str_mut_ptr) }));
}

#[test]
#[should_panic]
fn str_slice_index_panic_range() {
    let str_ref = "Hello, World!";

    assert_eq!("Hell", slice::SliceIndex::index(100..100, str_ref));
}

#[test]
#[should_panic]
fn str_slice_index_mut_panic_range() {
    let mut str_mut_bytes = [b'H', b'e', b'l', b'l', b'o'];
    let str_mut: &mut str = str::from_utf8_mut(&mut str_mut_bytes).unwrap();

    assert_eq!("Hell", slice::SliceIndex::index_mut(100..100, str_mut));
}

#[test]
#[should_panic]
fn str_slice_index_panic_range_to() {
    let str_ref = "Hello, World!";

    assert_eq!("Hell", slice::SliceIndex::index(..100, str_ref));
}

#[test]
#[should_panic]
fn str_slice_index_mut_panic_range_to() {
    let mut str_mut_bytes = [b'H', b'e', b'l', b'l', b'o'];
    let str_mut: &mut str = str::from_utf8_mut(&mut str_mut_bytes).unwrap();

    assert_eq!("Hell", slice::SliceIndex::index_mut(..100, str_mut));
}

#[test]
#[should_panic]
fn str_slice_index_panic_range_from() {
    let str_ref = "Hello, World!";

    assert_eq!("Hell", slice::SliceIndex::index(100.., str_ref));
}

#[test]
#[should_panic]
fn str_slice_index_mut_panic_range_from() {
    let mut str_mut_bytes = [b'H', b'e', b'l', b'l', b'o'];
    let str_mut: &mut str = str::from_utf8_mut(&mut str_mut_bytes).unwrap();

    assert_eq!("Hell", slice::SliceIndex::index_mut(100.., str_mut));
}
