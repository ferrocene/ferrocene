// covers:
// - `core::cell::RefCell::<T>::replace_with`
// - `core::cell::RefCell::<T>::take`
#[test]
fn ref_cell_replace_with() {
    let cell = core::cell::RefCell::new(false);

    cell.replace_with(|_| true);

    assert!(cell.take());
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
