// Don't use anything from core here.
// If you do, it will break once we start annotating more of core.
// You can still use traits as long as the impls use custom types.

//@ build-fail
//@ compile-flags -Z deduplicate-diagnostics=no

#![crate_type = "lib"]
#![no_std]
#![deny(ferrocene::uncertified)] //~ NOTE lint level

struct ExplicitDropImpl;
struct DropGlueOnly(ExplicitDropImpl);

#[ferrocene::prevalidated]
fn has_drop_unreachable() {
    DropGlueOnly(ExplicitDropImpl);
}

#[ferrocene::prevalidated] //~ NOTE marked
pub fn has_drop_reachable() { //~ NOTE validated
    DropGlueOnly(ExplicitDropImpl); //~ NOTE instantiated
}

// this span points to ptr::drop_in_place
//~? ERROR unvalidated

impl Drop for ExplicitDropImpl {
    fn drop(&mut self) {} //~ NOTE unvalidated
}
