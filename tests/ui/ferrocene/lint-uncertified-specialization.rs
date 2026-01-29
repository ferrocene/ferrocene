// Don't use anything from core here.
// If you do, it will break once we start annotating more of core.
// You can still use traits as long as the impls use custom types.

#![crate_type = "lib"]
#![deny(ferrocene::uncertified)]
#![feature(min_specialization)]

trait SpecializeMe {
    fn foo(&self) -> usize;
}

impl<T> SpecializeMe for T {
    #[ferrocene::prevalidated]
    default fn foo(&self) -> usize { 0 }
}

impl SpecializeMe for bool {
    fn foo(&self) -> usize { 1 }
}

#[ferrocene::prevalidated]
fn bar() {
    "xyz".foo(); // ok
    true.foo(); //~ ERROR unvalidated
}
