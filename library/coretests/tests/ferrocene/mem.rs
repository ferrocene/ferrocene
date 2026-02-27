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

// Test `<core::mem::Discriminant<T> as core::fmt::Debug>::fmt`.
#[test]
fn test_discriminant_debug_fmt() {
    let x = core::mem::discriminant(&Some(true));
    assert_eq!(format!("{x:?}"), "Discriminant(1)");
}

// Test `<core::mem::manually_drop::ManuallyDrop<T> as core::cmp::PartialEq>::eq`.
#[test]
fn test_manually_drop_eq() {
    let mut x = core::mem::ManuallyDrop::new(vec![]);
    let mut y = core::mem::ManuallyDrop::new(vec![1]);

    x.push(1);
    assert_eq!(x, y);

    unsafe {
        core::mem::ManuallyDrop::drop(&mut x);
        core::mem::ManuallyDrop::drop(&mut y);
    }
}

// Test `core::mem::maybe_dangling::MaybeDangling::<P>::into_inner`.
#[test]
fn test_maybe_dangling_into_inner() {
    let boxed = Box::new(0u32);
    let maybe_dangling = core::mem::MaybeDangling::new(boxed);
    assert_eq!(*maybe_dangling.into_inner(), 0u32);
}
