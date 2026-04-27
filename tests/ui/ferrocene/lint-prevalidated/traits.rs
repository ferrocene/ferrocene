//@ revisions: pre-mono post-mono
//@[pre-mono]check-fail
//@[post-mono]build-fail

#![deny(ferrocene::unvalidated)]
#![crate_type = "lib"]

trait Trait {
    fn default_unvalidated() {}
    #[ferrocene::prevalidated]
    fn default_validated() {}

    fn unvalidated();
    #[ferrocene::prevalidated]
    fn validated();
}

struct MissingValidations;
#[cfg(pre_mono)]
impl Trait for MissingValidations {
    fn default_validated() {} //~ ERROR unvalidated
    fn unvalidated() {}
    fn validated() {} //~ ERROR unvalidated
}

struct HasValidations;
impl Trait for HasValidations {
    #[ferrocene::prevalidated]
    fn unvalidated() {}
    #[ferrocene::prevalidated]
    fn validated() {}
}

#[cfg(pre_mono)]
#[ferrocene::prevalidated]
fn direct_checks() {
    // MissingValidations::default_unvalidated(); //~ ERROR unvalidated
    // MissingValidations::default_validated(); //~ ERROR unvalidated
    // MissingValidations::unvalidated(); //~[pre-mono] ERROR unvalidated

    // HasValidations::default_validated(); // OK
    // HasValidations::default_unvalidated(); //~ ERROR unvalidated
    // HasValidations::validated(); // OK
    HasValidations::unvalidated(); // OK: even though the trait isn't validated, the impl is.
}

#[cfg(pre_mono)]
#[ferrocene::prevalidated]
fn generics_checks<T: Trait>(_: T) {
    T::default_unvalidated(); //[pre-mono]~ ERROR unvalidated
    T::unvalidated(); //[pre-mono]~ ERROR unvalidated
    T::validated(); // ok
    T::default_validated(); // ok
}

#[ferrocene::prevalidated]
fn call(f: impl Fn()) {
    #![warn(ferrocene::possibly_unvalidated)]
    f(); //~ WARN unvalidated
}

#[ferrocene::prevalidated]
pub fn indirect_checks() { // only checked in post-mono
    call(HasValidations::default_validated); // OK
    call(HasValidations::validated); // OK
    call(HasValidations::unvalidated); // OK: even though the trait isn't validated, the impl is
    call(HasValidations::default_unvalidated); //~ ERROR unvalidated
}
