use core::clone::CloneToUninit;
use core::mem::MaybeUninit;

#[test]
#[allow(suspicious_double_ref_op)]
fn test_borrowed_clone() {
    let x = 5;
    let y: &i32 = &x;
    let z: &i32 = (&y).clone();
    assert_eq!(*z, 5);
}

#[test]
fn test_clone_from() {
    let a = Box::new(5);
    let mut b = Box::new(10);
    b.clone_from(&a);
    assert_eq!(*b, 5);
}

#[test]
fn test_clone_to_uninit_slice_success() {
    // Using `String`s to exercise allocation and Drop of the individual elements;
    // if something is aliased or double-freed, at least Miri will catch that.
    let a: [String; 3] = ["a", "b", "c"].map(String::from);

    let mut storage: MaybeUninit<[String; 3]> = MaybeUninit::uninit();
    let b: [String; 3] = unsafe {
        a[..].clone_to_uninit(storage.as_mut_ptr() as *mut [String]);
        storage.assume_init()
    };

    assert_eq!(a, b);
}

#[test]
#[cfg(panic = "unwind")]
fn test_clone_to_uninit_slice_drops_on_panic() {
    use core::sync::atomic::AtomicUsize;
    use core::sync::atomic::Ordering::Relaxed;

    /// A static counter is OK to use as long as _this one test_ isn't run several times in
    /// multiple threads.
    static COUNTER: AtomicUsize = AtomicUsize::new(0);
    /// Counts how many instances are live, and panics if a fifth one is created
    struct CountsDropsAndPanics {}
    impl CountsDropsAndPanics {
        fn new() -> Self {
            COUNTER.fetch_add(1, Relaxed);
            Self {}
        }
    }
    impl Clone for CountsDropsAndPanics {
        fn clone(&self) -> Self {
            if COUNTER.load(Relaxed) == 4 { panic!("intentional panic") } else { Self::new() }
        }
    }
    impl Drop for CountsDropsAndPanics {
        fn drop(&mut self) {
            COUNTER.fetch_sub(1, Relaxed);
        }
    }

    let a: [CountsDropsAndPanics; 3] = core::array::from_fn(|_| CountsDropsAndPanics::new());
    assert_eq!(COUNTER.load(Relaxed), 3);

    let panic_payload = std::panic::catch_unwind(|| {
        let mut storage: MaybeUninit<[CountsDropsAndPanics; 3]> = MaybeUninit::uninit();
        // This should panic halfway through
        unsafe {
            a[..].clone_to_uninit(storage.as_mut_ptr() as *mut [CountsDropsAndPanics]);
        }
    })
    .unwrap_err();
    assert_eq!(panic_payload.downcast().unwrap(), Box::new("intentional panic"));

    // Check for lack of leak, which is what this test is looking for
    assert_eq!(COUNTER.load(Relaxed), 3, "leaked during clone!");

    // Might as well exercise the rest of the drops
    drop(a);
    assert_eq!(COUNTER.load(Relaxed), 0);
}
