// <&mut T as core::borrow::Borrow<T>>::borrow
#[test]
fn mut_ref_borrow() {
    let mut x: u64 = 1;
    let y: &mut u64 = &mut x;
    let z = y as &mut u64;
    let borrowed: &u64 = core::borrow::Borrow::borrow(&z);
    assert_eq!(*borrowed, 1);
}
