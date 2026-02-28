// Make sure the same diagnostics are emitted regardless of hashing order.
// A previous implementation sometimes showed the instantiation site as `reachable_indirect` and
// sometimes directly as `Validated::drop`.

//@ build-fail

#![crate_type = "lib"]
#![deny(ferrocene::unvalidated)] //~ NOTE lint level

fn unvalidated() {} //~ NOTE unvalidated

struct Validated;
impl Drop for Validated {
    #[ferrocene::prevalidated] //~ NOTE marked
    fn drop(&mut self) { //~ NOTE validated
        unvalidated(); //~ ERROR calls
    }
}

#[ferrocene::prevalidated]
fn uninstantiated_generic_indirect<T>(_val: T) {
}

#[ferrocene::prevalidated]
pub fn reachable_indirect() {
    uninstantiated_generic_indirect(Validated);
}
