use core::cell;

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
