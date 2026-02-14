// Make sure the post-mono pass recurses even if the lint is allowed at the outer level.

//@ build-fail

#![crate_type = "lib"]
#![deny(ferrocene::uncertified)] //~ NOTE lint level

fn unvalidated() {} //~ NOTE unvalidated

struct Validated;
impl Drop for Validated {
    #[ferrocene::prevalidated]
    fn drop(&mut self) {
        unvalidated(); //~ ERROR calls
    }
}

#[ferrocene::prevalidated]
fn uninstantiated_generic_indirect<T>(val: T) {
} //~ NOTE instantiated

#[ferrocene::prevalidated] //~ NOTE marked
#[allow(ferrocene::uncertified)]
pub fn reachable_allowed() { //~ NOTE validated
    uninstantiated_generic_indirect(Validated);
}
