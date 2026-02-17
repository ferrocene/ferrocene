use core::sync::atomic;

#[test]
fn atomic_methods() {
    use atomic::{AtomicU32, Ordering::*};
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

    let _ = atomic::AtomicU32::from_mut(arr.get_mut(0).unwrap()).get_mut();
    let _ = AtomicU32::get_mut_slice(AtomicU32::from_mut_slice(arr.get_mut(0..1).unwrap()));

    let _ = unsafe { AtomicU32::from_ptr(arr.get_mut(0).unwrap() as *mut u32) };
}

macro_rules! test_atomic_compare_and_swap {
    ($($fn:ident => $width:literal => $atomic_t:ty,)*) => { $(
        #[test]
        #[expect(deprecated)]
        fn $fn() {
            let atomic = <$atomic_t>::new(5);

            assert_eq!(atomic.compare_and_swap(5, 10, atomic::Ordering::Relaxed), 5); // success
            assert_eq!(atomic.compare_and_swap(20, 30, atomic::Ordering::Relaxed), 10); // failure
        }
    )*};
}

test_atomic_compare_and_swap!(
    atomic_u8_compare_and_swap  => "8" => atomic::AtomicU8,
    atomic_u16_compare_and_swap  =>"16" => atomic::AtomicU16,
    atomic_u32_compare_and_swap =>"32" => atomic::AtomicU32,
    atomic_usize_compare_and_swap => "ptr" => atomic::AtomicUsize,
    atomic_i8_compare_and_swap => "8"=> atomic::AtomicI8,
    atomic_i16_compare_and_swap => "16"=> atomic::AtomicI16,
    atomic_i32_compare_and_swap => "32"=> atomic::AtomicI32,
    atomic_isize_compare_and_swap => "ptr"=> atomic::AtomicIsize,
);

#[cfg(target_has_atomic_load_store = "64")]
test_atomic_compare_and_swap!(
    atomic_u64_compare_and_swap => "64"=> atomic::AtomicU64,
    atomic_i64_compare_and_swap => "64"=> atomic::AtomicI64,
);

#[test]
#[expect(deprecated)]
fn atomic_bool_compare_and_swap() {
    let atomic = atomic::AtomicBool::new(false);

    assert_eq!(atomic.compare_and_swap(false, true, atomic::Ordering::Relaxed), false); // success
    assert_eq!(atomic.compare_and_swap(false, true, atomic::Ordering::Relaxed), true); // failure

    assert_eq!(atomic.compare_and_swap(true, false, atomic::Ordering::Release), true); // success
    assert_eq!(atomic.compare_and_swap(true, false, atomic::Ordering::Release), false); // failure

    assert_eq!(atomic.compare_and_swap(false, true, atomic::Ordering::SeqCst), false); // success
    assert_eq!(atomic.compare_and_swap(false, true, atomic::Ordering::SeqCst), true); // failure

    assert_eq!(atomic.compare_and_swap(true, false, atomic::Ordering::Acquire), true); // success
    assert_eq!(atomic.compare_and_swap(true, false, atomic::Ordering::Acquire), false); // failure

    assert_eq!(atomic.compare_and_swap(false, true, atomic::Ordering::AcqRel), false); // success
    assert_eq!(atomic.compare_and_swap(false, true, atomic::Ordering::AcqRel), true); // failure
}

#[test]
#[expect(deprecated)]
#[cfg(target_has_atomic = "ptr")]
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
#[should_panic = "there is no such thing as a relaxed fence"]
#[expect(invalid_atomic_ordering)]
fn atomic_compiler_fence_relaxed() {
    atomic::compiler_fence(atomic::Ordering::Relaxed);
}

// covers `core::sync::atomic::fence`.
#[test]
fn atomic_fence() {
    use atomic::Ordering::*;

    atomic::fence(Acquire);
    atomic::fence(Release);
    atomic::fence(AcqRel);
    atomic::fence(SeqCst);
}

// covers `core::sync::atomic::fence`.
#[test]
#[should_panic = "there is no such thing as a relaxed fence"]
#[expect(invalid_atomic_ordering)]
fn atomic_fence_relaxed() {
    atomic::fence(atomic::Ordering::Relaxed);
}

// covers `core::sync::atomic::<$atomic_t>::fetch_update`
macro_rules! test_atomic_fetch_update {
    ($($fn:ident => $atomic_t:ty,)*) => { $(
        #[test]
        fn $fn() {
            let atomic = <$atomic_t>::new(5);

            assert_eq!(atomic.fetch_update(atomic::Ordering::Relaxed, atomic::Ordering::Relaxed, |val| { if val == 5 { Some(10) } else { None } }), Ok(5)); // success
            assert_eq!(atomic.fetch_update(atomic::Ordering::Relaxed, atomic::Ordering::Relaxed, |val| { if val == 5 { Some(10) } else { None } }), Err(10)); // failure
        }
    )*};
}
test_atomic_fetch_update!(
    atomic_u8_fetch_update => atomic::AtomicU8,
    atomic_u16_fetch_update => atomic::AtomicU16,
    atomic_u32_fetch_update => atomic::AtomicU32,
    atomic_usize_fetch_update => atomic::AtomicUsize,
);

// covers `<core::sync::atomic::<$atomic_t>:: as core::fmt::Debug>::fmt`
macro_rules! test_atomic_debug_fmt {
    ($($(#[$attr:meta])* $fn:ident => $atomic_t:ty : $val:literal == $str:literal,)*) => { $(
        #[test]
        $(#[$attr])*
        fn $fn() {
            let atomic = <$atomic_t>::new($val);

            assert_eq!(format!("{atomic:?}"), $str);
        }
    )*};
}
test_atomic_debug_fmt!(
    atomic_u8_debug_fmt => atomic::AtomicU8: 5 == "5",
    atomic_u16_debug_fmt => atomic::AtomicU16: 5 == "5",
    atomic_u32_debug_fmt => atomic::AtomicU32: 5 == "5",
    #[cfg(target_has_atomic = "64")]
    atomic_u64_debug_fmt => atomic::AtomicU64: 5 == "5",
    atomic_usize_debug_fmt => atomic::AtomicUsize: 5 == "5",
    atomic_bool_debug_fmt => atomic::AtomicBool: true == "true",
);
