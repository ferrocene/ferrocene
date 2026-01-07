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

// covers:
// - `core::cell::RefCell::<T>::borrow`
// - `core::cell::panic_already_mutably_borrowed`
#[test]
#[should_panic]
fn already_borrowed_ref_cell_panics() {
    let cell = core::cell::RefCell::new(false);

    cell.replace_with(|_| !*cell.borrow());
}
