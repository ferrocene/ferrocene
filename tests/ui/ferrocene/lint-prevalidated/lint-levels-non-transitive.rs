// Make sure that `allow` only applies to the call site, not to nested instantiation sites.

//@ build-fail

#![crate_type = "lib"]
#![deny(ferrocene::unvalidated)] //~ NOTE lint level

fn unvalidated() {} //~ NOTE unvalidated
fn unvalidated2() {}


#[ferrocene::prevalidated]
fn uninstantiated_generic_indirect(f: impl FnOnce()) {
    f(); //~ ERROR unvalidated
}

#[ferrocene::prevalidated] //~ NOTE marked
#[allow(ferrocene::unvalidated)]
pub fn reachable_indirect() { //~ NOTE is validated
    uninstantiated_generic_indirect(unvalidated); //~ NOTE instantiated
    unvalidated2(); // ok
}
