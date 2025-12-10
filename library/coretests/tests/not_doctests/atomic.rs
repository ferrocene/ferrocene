use core::sync::atomic;

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

    #[cfg(target_has_atomic = "8")]
    test_atomic_compare_and_swap!(atomic::AtomicU8);
    #[cfg(target_has_atomic = "16")]
    test_atomic_compare_and_swap!(atomic::AtomicU16);
    #[cfg(target_has_atomic = "32")]
    test_atomic_compare_and_swap!(atomic::AtomicU32);
    #[cfg(target_has_atomic = "64")]
    test_atomic_compare_and_swap!(atomic::AtomicU64);
    #[cfg(target_has_atomic = "ptr")]
    test_atomic_compare_and_swap!(atomic::AtomicUsize);
    #[cfg(target_has_atomic = "8")]
    test_atomic_compare_and_swap!(atomic::AtomicI8);
    #[cfg(target_has_atomic = "16")]
    test_atomic_compare_and_swap!(atomic::AtomicI16);
    #[cfg(target_has_atomic = "32")]
    test_atomic_compare_and_swap!(atomic::AtomicI32);
    #[cfg(target_has_atomic = "64")]
    test_atomic_compare_and_swap!(atomic::AtomicI64);
    #[cfg(target_has_atomic = "ptr")]
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
