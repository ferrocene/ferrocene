// Don't use anything from core here.
// If you do, it will break once we start annotating more of core.
// You can still use traits as long as the impls use custom types.

//@ build-fail

#![crate_type = "lib"]
#![no_std]
#![deny(ferrocene::unvalidated)] //~ NOTE lint level

struct ExplicitDropImpl;

#[ferrocene::prevalidated]
fn has_drop_unreachable() {
    ExplicitDropImpl;
}

#[ferrocene::prevalidated] //~ NOTE marked
pub fn has_drop_reachable() { //~ NOTE validated
    ExplicitDropImpl; //~ NOTE instantiated by
}

//~? ERROR unvalidated

impl Drop for ExplicitDropImpl {
    fn drop(&mut self) {} //~ NOTE unvalidated
}
