// Don't use anything from core here.
// If you do, it will break once we start annotating more of core.
// You can still use traits as long as the impls use custom types.
// In particular do NOT use numerics anywhere here,
// nor any syntax in core::ops unless it's with an ADT.

//@ revisions: dedup no-dedup
//@[dedup] compile-flags: -Z deduplicate-diagnostics=yes

#![crate_type = "lib"]
#![deny(ferrocene::unvalidated)]

use std::clone::Clone;
use std::cmp::{PartialEq, PartialOrd};

fn normal_def() {
    normal_def2(); // ok
}

fn normal_def2() {}

#[ferrocene::prevalidated]
const fn marked_validated() {}

const fn const_fn() {}

extern "C" { fn extern_fn(); }

macro_rules! mbe {
    () => { normal_def() }
    //[no-dedup]~^ ERROR unvalidated
}

struct Ctor;

#[derive(Clone)]
struct Unvalidated;
#[derive(Clone)]
struct Unvalidated2;

#[ferrocene::prevalidated]
#[derive(Clone)]
struct Validated;

impl Unvalidated {
    fn inherent_fn(&self) {}
    fn generic_inherent_fn<T>(x: T) {}
}

impl Copy for Unvalidated {}
impl Copy for Validated {}

impl PartialOrd for Unvalidated {
    fn partial_cmp(&self, _: &Self) -> Option<std::cmp::Ordering> {
        None
    }
}

impl PartialEq<Unvalidated> for Unvalidated {
    #[ferrocene::prevalidated]
    fn eq(&self, _: &Self) -> bool {
        true
    }

    fn ne(&self, _: &Self) -> bool {
        false
    }
}

impl ToString for Validated {
    #[ferrocene::prevalidated]
    fn to_string(&self) -> String {
        String::new()
    }
}

trait FancyClone {
    fn fancy_clone();
}
impl<T: Clone> FancyClone for T {
    fn fancy_clone() {}
}

const UNCERTIFIED_CLOSURE_CONST: fn() = || normal_def();
static UNCERTIFIED_CLOSURE_STATIC: fn() = || normal_def();
const UNCERTIFIED_DYN_CONST: &'static (dyn Sync + PartialOrd<Unvalidated>) = &Unvalidated;
static UNCERTIFIED_DYN_STATIC: &'static (dyn Sync + PartialOrd<Unvalidated>) = &Unvalidated;

#[ferrocene::prevalidated]
const CERTIFIED_CLOSURE_CONST: fn() = || normal_def(); //~ ERROR unvalidated
#[ferrocene::prevalidated]
static CERTIFIED_CLOSURE_STATIC: fn() = || normal_def(); //~ ERROR unvalidated
#[ferrocene::prevalidated]
const CERTIFIED_DYN_CONST: &'static (dyn Sync + PartialOrd<Unvalidated>) = &Unvalidated;
//~^ ERROR unvalidated
#[ferrocene::prevalidated]
static CERTIFIED_DYN_STATIC: &'static (dyn Sync + PartialOrd<Unvalidated>) = &Unvalidated;
//~^ ERROR unvalidated

#[ferrocene::prevalidated]
fn validated() {
    normal_def(); //~ ERROR unvalidated
    let fn_ptr: fn() = normal_def; //[no-dedup]~ ERROR unvalidated
    fn_ptr(); // ok

    let trait_fn_type = <Unvalidated as Clone>::clone;
    trait_fn_type(&Unvalidated); //~ ERROR unvalidated
    let trait_fn_ptr: fn(&Unvalidated) -> Unvalidated = trait_fn_type; //[no-dedup]~ ERROR unvalidated
    trait_fn_ptr(&Unvalidated); // ok

    let dyn_trait: &dyn PartialOrd<Unvalidated> = &Unvalidated; //~ ERROR unvalidated
    dyn_trait.partial_cmp(&Unvalidated); // ok

    // "vector table", common in embedded systems
    unsafe {
        let ptr_to_int = core::mem::transmute::<_, usize>(normal_def as *const ());
        //[no-dedup]~^ ERROR possibly calls
        (core::mem::transmute::<usize, fn()>(ptr_to_int))();
    }

    let dyn_trait_partially_validated: &dyn PartialEq<Unvalidated> = &Unvalidated;
    //~^ ERROR unvalidated
    dyn_trait_partially_validated.eq(&Unvalidated); // ok
    dyn_trait_partially_validated.ne(&Unvalidated); // ok

    let dyn_trait_fully_validated: &dyn ToString = &Validated; // ok: this impl is validated
    dyn_trait_fully_validated.to_string(); // ok

    Ctor; // ok
    mbe!(); // caught in macro definition above; maybe we should have both spans?
    Unvalidated.inherent_fn(); //~ ERROR unvalidated
    Unvalidated.clone();
    //[no-dedup]~^ ERROR unvalidated
   Unvalidated::generic_inherent_fn::<usize>(1); //~ ERROR unvalidated

    marked_validated(); // ok

    UNCERTIFIED_CONST; // ok
    UNCERTIFIED_CLOSURE_CONST(); //~ ERROR unvalidated
    UNCERTIFIED_CLOSURE_STATIC(); //~ ERROR unvalidated
    UNCERTIFIED_DYN_CONST.partial_cmp(&Unvalidated); //~ ERROR unvalidated
    UNCERTIFIED_DYN_STATIC.partial_cmp(&Unvalidated); //~ ERROR unvalidated

    CERTIFIED_CLOSURE_CONST(); // ok
    CERTIFIED_CLOSURE_STATIC(); // ok
    CERTIFIED_DYN_CONST.partial_cmp(&Unvalidated); // ok
    CERTIFIED_DYN_STATIC.partial_cmp(&Unvalidated); // ok

    use normal_def as rename;
    rename();
    //[no-dedup]~^ ERROR unvalidated

    // FIXME: lint all unsafe blocks
    // FIXME: lint calls to `extern "C"` functions
}

#[ferrocene::prevalidated]
const CERTIFIED_CONST2: () = marked_validated(); // ok

const UNCERTIFIED_CONST: () = const_fn(); // ok
