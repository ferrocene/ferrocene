// covers `core::slice::<impl [T]>::first_mut`
#[test]
fn first_mut() {
    let mut empty = [0u8; 0];
    assert!(empty.first_mut().is_none());

    let mut arr = [0u8; 5];
    *arr.first_mut().unwrap() = 1;

    assert_eq!(arr[0], 1);
}

// covers:
// - `core::slice::<impl [T]>::clone_from_slice`
// - `core::slice::<impl [T]>::split_last`
// - `core::slice::<impl [T]>::split_last_mut`
#[test]
fn split_last() {
    let mut empty = [0u8; 0];

    assert!(empty.split_last().is_none());
    assert!(empty.split_last_mut().is_none());

    let a1 = [1u8; 5];
    let mut a2 = [0u8; 5];

    let (h1, t1) = a1.split_last().unwrap();
    let (h2, t2) = a2.split_last_mut().unwrap();

    *h2 = *h1;
    assert_eq!(*h2, 1);

    t2.clone_from_slice(t1);
    assert_eq!(a1, a2);
}

// covers `core::slice::<impl [T]>::split_at`
#[test]
#[should_panic = "mid > len"]
fn split_at_panics() {
    let _ = [1u8; 5].split_at(6);
}

// covers `core::slice::<impl [T]>::split_at_mut`
#[test]
#[should_panic = "mid > len"]
fn split_at_mut_panics() {
    let _ = [1u8; 5].split_at_mut(6);
}

// covers `core::slice::<impl [T]>::align_to_mut`
#[test]
fn align_to_mut() {
    let mut zst_arr = [(); 5];
    assert_eq!(
        unsafe { zst_arr.align_to_mut::<u8>() },
        ([(); 5].as_mut_slice(), [].as_mut_slice(), [].as_mut_slice())
    );

    #[repr(C, align(256))]
    #[derive(Debug, PartialEq)]
    struct Foo([u8; 256]);

    let mut arr = [0u8; 1];
    assert_eq!(
        unsafe { arr.align_to_mut::<Foo>() },
        ([0u8; 1].as_mut_slice(), [].as_mut_slice(), [].as_mut_slice())
    );
}

// covers `core::slice::rotate::ptr_rotate`
#[test]
fn rotate_zst() {
    let mut zst_arr = [(); 5];

    zst_arr.rotate_left(5);
}
