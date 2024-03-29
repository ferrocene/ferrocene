//@ run-pass
// Test when destructors run in a for loop. The intention is
// that the value for each iteration is dropped *after* the loop
// body has executed. This is true even when the value is assigned
// to a `_` pattern (and hence ignored).

use std::cell::Cell;

struct Flag<'a>(&'a Cell<bool>);

impl<'a> Drop for Flag<'a> {
    fn drop(&mut self) {
        self.0.set(false)
    }
}

fn main() {
    let alive2 = Cell::new(true);
    for _i in std::iter::once(Flag(&alive2)) {
        // The Flag value should be alive in the for loop body
        assert_eq!(alive2.get(), true);
    }
    // The Flag value should be dead outside of the loop
    assert_eq!(alive2.get(), false);

    let alive = Cell::new(true);
    for _ in std::iter::once(Flag(&alive)) {
        // The Flag value should be alive in the for loop body even if it wasn't
        // bound by the for loop
        assert_eq!(alive.get(), true);
    }
    // The Flag value should be dead outside of the loop
    assert_eq!(alive.get(), false);
}

// ferrocene-annotations: fls_u2mzjgiwbkz0
// Destructors
//
// ferrocene-annotations: fls_afafmafz4hf2
// Drop Order
//
// ferrocene-annotations: fls_5eima0pd31c0
// Drop Scope Extension
//
// ferrocene-annotations: fls_rm4ncoopcdvj
// Drop Scopes
//
// ferrocene-annotations: fls_onfyolkcbeh3
// For Loops
//
// ferrocene-annotations: fls_cleoffpn5ew6
// Temporaries
//
// ferrocene-annotations: fls_gho955gmob73
// Variables
