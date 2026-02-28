#![deny(ferrocene::unvalidated)] //~ NOTE defined here
#![crate_type = "lib"]
// want to extend this to modules
// FIXME: this doesn't work at crate level, "custom inner attributes are unstable"
// and it has to be that way, see
// https://github.com/rust-lang/rfcs/pull/3808#discussion_r2616622459,
// and https://github.com/rust-lang/rust/issues/54726#issuecomment-431931811 for a possible
// way to relax the limitation.
// #![ferrocene::prevalidated]

//@ build-fail
//@ revisions: dedup no-dedup
//@[dedup] compile-flags: -Z deduplicate-diagnostics=yes

struct Unvalidated;
struct Validated;

impl Clone for Unvalidated {
    fn clone(&self) -> Self { Unvalidated }
    //~^ NOTE is unvalidated
    //[no-dedup]~^^ NOTE is unvalidated
    fn clone_from(&mut self, other: &Self) { //~ NOTE unvalidated
        *self = other.clone();
        //[no-dedup]~^ ERROR calls an unvalidated
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

// has to be checked post-mono
#[ferrocene::prevalidated]
fn uninstantiated_generic<T: Clone + Default>(x: T) {
    x.clone(); //~ ERROR calls an unvalidated
    let fn_type = T::default; // ok -- not actually called, and can't be resolved by THIR pass
    let mut y = fn_type(); //~ ERROR unvalidated

    let fn_ptr: fn(&mut T, &T) = T::clone_from; //~ ERROR unvalidated
    //~^ NOTE cast to a function pointer
    //~^^ NOTE must assume
    //[no-dedup]~^^^ NOTE instantiated by
    fn_ptr(&mut y, &x); // ok
}

#[ferrocene::prevalidated]
fn only_instantiated_by_unvalidated<T: Into<()>>(val: T) {
    val.into() // ok: not called from a validated root
}

fn reachable_unvalidated() {
    only_instantiated_by_unvalidated(());
}

#[ferrocene::prevalidated] //~ NOTE marked
pub fn instantiation_reachable() { //~ NOTE is validated
    uninstantiated_generic(Unvalidated);
    //~^ NOTE instantiated by
    //~^^ NOTE validated entrypoint
    uninstantiated_generic(Validated); // ok
}

// Currently *not* checked.
// Nadrierel says it should be possible to lift this restriction by not running the
// `SimplifyCfg-initial` MIR passes, if that's something we want. I think we would also have to
// modify our RootCollector to use the Eager collection strategy.
// But right now we just ban our customers from having dead code, lol.
#[ferrocene::prevalidated]
fn instantiation_unreachable() {
    uninstantiated_generic(Unvalidated); // ok
    uninstantiated_generic(Validated); // ok
}
