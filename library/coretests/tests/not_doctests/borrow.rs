// <&mut T as core::borrow::Borrow<T>>::borrow
#[test]
fn mut_ref_borrow() {
    let mut x: u64 = 1;
    let y: &mut u64 = &mut x;
    let z = y as &mut u64;
    let borrowed: &u64 = core::borrow::Borrow::borrow(&z);
    assert_eq!(*borrowed, 1);
}

// <&mut T as core::borrow::BorrowMut<T>>::borrow_mut
#[test]
fn mut_ref_borrow_mut() {
    let mut x: u64 = 1;
    let y: &mut u64 = &mut x;
    let mut z = y as &mut u64;
    let borrowed: &u64 = core::borrow::BorrowMut::borrow_mut(&mut z);
    assert_eq!(*borrowed, 1);
}
