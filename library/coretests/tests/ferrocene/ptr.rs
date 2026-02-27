#[test]
fn try_cast_aligned() {
    let x = 0u64;

    let aligned: *const u64 = &x;
    let unaligned = unsafe { aligned.byte_add(1) };

    assert!(aligned.try_cast_aligned::<u32>().is_some());
    assert!(unaligned.try_cast_aligned::<u32>().is_none());
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
    // SAFETY: `ptr` is a pointer to a ZST so subtracting anything from it is a noop.
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
