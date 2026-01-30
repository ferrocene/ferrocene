// Unlike most other lints, post-mono lint levels are controlled at the *instantiation* site, not at
// the place where an uncertified function is actually called.

//@ build-pass

#![crate_type = "lib"]
// NOTE: never takes effect, always overridden by a more specific lint control
#![deny(ferrocene::uncertified)]

fn unvalidated() {}
//~^ NOTE unvalidated

struct Unvalidated;
impl Drop for Unvalidated {
    fn drop(&mut self) {} //~ NOTE unvalidated
}

impl Clone for Unvalidated {
    fn clone(&self) -> Self { Unvalidated } //~ NOTE unvalidated
}

#[ferrocene::prevalidated]
#[warn(ferrocene::uncertified)] //~ NOTE lint level
fn uninstantiated_generic_indirect<T>(val: T) {} //~ NOTE instantiated

#[ferrocene::prevalidated]
fn uninstantiated_generic_direct<T: Clone>(val: T) {
    #[warn(ferrocene::uncertified)] //~ NOTE lint level
    val.clone(); //~ WARN calls
}

// this span points to core::ptr::drop_in_place
//~? WARN calls an unvalidated

#[ferrocene::prevalidated] //~ NOTE marked
pub fn reachable() { //~ NOTE validated
    uninstantiated_generic_indirect(Unvalidated);
    #[warn(ferrocene::uncertified)]
    uninstantiated_generic_direct(Unvalidated); //~ NOTE instantiated
    // no-op: lint already emitted
    // FIXME: ideally this would error now, since there's no lint control here.
    // but caching that correctly is non-trivial.
    uninstantiated_generic_direct(Unvalidated);
}

#[ferrocene::prevalidated] //~ NOTE marked
fn unreachable() { //~ NOTE is validated
    #[warn(ferrocene::uncertified)] //~ NOTE lint level
    unvalidated();
    //~^ WARN calls
}

// TODO: make sure this recurses even if lint is allowed
