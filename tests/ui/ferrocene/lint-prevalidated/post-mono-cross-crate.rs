#![deny(ferrocene::uncertified)] //~ NOTE defined here
#![crate_type = "lib"]

//@ build-fail
//@ aux-build: uncertified-post-mono.rs
//@ revisions: dedup no-dedup
//@[dedup] compile-flags: -Z deduplicate-diagnostics=yes

extern crate uncertified_post_mono;
use uncertified_post_mono::uninstantiated_generic;

//~? ERROR calls an unvalidated method
//~? ERROR calls an unvalidated associated function
//~? ERROR possibly calls an unvalidated method
//~? NOTE: cast to a function pointer
//~? NOTE: must assume

#[ferrocene::prevalidated] //~ NOTE marked
pub fn instantiation_reachable() { //~ NOTE is validated
    uninstantiated_generic(Unvalidated);
    //~^ NOTE instantiated by
    //~^^ NOTE validated entrypoint
    uninstantiated_generic(Validated); // ok
}

struct Unvalidated;
struct Validated;

impl Clone for Unvalidated {
    fn clone(&self) -> Self { Unvalidated }
    //~^ NOTE is unvalidated
    //[no-dedup]~^^ NOTE is unvalidated
    fn clone_from(&mut self, other: &Self) { //~ NOTE unvalidated
        *self = other.clone();
        //[no-dedup]~^ ERROR calls an unvalidated
        //[no-dedup]~^^ NOTE instantiated by
    }
}
impl Clone for Validated {
    #[ferrocene::prevalidated]
    fn clone(&self) -> Self { Validated }
    #[ferrocene::prevalidated]
    fn clone_from(&mut self, other: &Self) {
        *self = other.clone();
    }
}

impl Default for Unvalidated {
    fn default() -> Self { Unvalidated } //~ NOTE unvalidated
}

impl Default for Validated {
    #[ferrocene::prevalidated]
    fn default() -> Self { Validated }
}
