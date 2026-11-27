// Ferrocene addition: The failing query stack in this test differs between panic=unwind and
// panic=abort configurations, leading to a different set of error messages
//
//@ revisions: unwind abort
//@ [unwind] needs-unwind
//@ [abort] compile-flags: -C panic=abort

use std::cell::Cell;
use std::rc::Rc;
//@ ignore-parallel-frontend query cycle
fn send<T: Send>(_: T) {}

fn main() {}

// Cycles should work as the deferred obligations are
// independently resolved and only require the concrete
// return type, which can't depend on the obligation.
fn cycle1() -> impl Clone { //~ ERROR: cycle detected
    send(cycle2().clone());

    Rc::new(Cell::new(5))
}

fn cycle2() -> impl Clone {
    send(cycle1().clone());

    Rc::new(String::from("foo"))
}

// ferrocene-annotations: fls_eiw4by8z75di
// Send and Sync
