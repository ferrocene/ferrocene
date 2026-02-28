// Don't use anything from core here.
// If you do, it will break once we start annotating more of core.
// You can still use traits as long as the impls use custom types.
// In particular do NOT use numerics anywhere here,
// nor any syntax in core::ops unless it's with an ADT.

//@ compile-flags: -Zdeduplicate-diagnostics=yes

#![crate_type = "lib"]
#![no_std]
#![deny(ferrocene::unvalidated)]

fn normal_def() {}

#[derive(Clone)]
struct Unvalidated;
#[derive(Clone)]
struct Unvalidated2;

impl Copy for Unvalidated {}

impl Unvalidated {
    fn inherent_fn(&self) {}
}

trait FancyClone {
    fn fancy_clone();
}
impl<T: Clone> FancyClone for T {
    fn fancy_clone() {}
}

#[ferrocene::prevalidated]
fn deduplication() {
    normal_def(); //~ ERROR unvalidated
    normal_def(); // ok (duplicate)

    Unvalidated.inherent_fn(); //~ ERROR unvalidated
    Unvalidated.inherent_fn(); // ok (duplicate)

    // blanket impl
    Unvalidated::fancy_clone(); //~ ERROR unvalidated
    Unvalidated2::fancy_clone(); // ok (duplicate)
}
