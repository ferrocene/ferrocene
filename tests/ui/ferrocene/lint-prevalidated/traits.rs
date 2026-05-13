//@ revisions: pre-mono post-mono
//@[pre-mono]check-fail
//@[post-mono]build-fail

#![deny(ferrocene::unvalidated)] //~ NOTE lint level
#![crate_type = "lib"]

trait Trait {
    fn unvalidated();
    fn default_unvalidated() {}

    #[ferrocene::requires_validation] //[pre-mono]~ NOTE marked
    fn validated();

    #[ferrocene::prevalidated] //[pre-mono]~ NOTE marked
    fn default_is_validated() {}
}

struct MissingValidations;
#[cfg(pre_mono)]
impl Trait for MissingValidations {
    fn default_is_validated() {} //[pre-mono]~ ERROR unvalidated
    //[pre-mono]~^ NOTE require every impl
    //[pre-mono]~^^ NOTE known_unvalidated
    fn unvalidated() {}
    fn validated() {} //[pre-mono]~ ERROR unvalidated
    //[pre-mono]~^ NOTE require every impl
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
    // MissingValidations::default_unvalidated(); // ERROR unvalidated
    // MissingValidations::unvalidated(); //[pre-mono] ERROR unvalidated

    // HasValidations::default_unvalidated(); // ERROR unvalidated
    // HasValidations::validated(); // OK
    HasValidations::unvalidated(); // OK: even though the trait isn't validated, the impl is.
}

#[cfg(pre_mono)]
#[ferrocene::prevalidated] //[pre-mono]~ NOTE marked
fn generics_checks<T: Trait>(_: T) { //[pre-mono]~ NOTE validated
    T::default_unvalidated(); //[pre-mono]~ ERROR unvalidated
    //[pre-mono]~^ NOTE does not know
    //[pre-mono]~^^ NOTE possibly_unvalidated
    T::unvalidated(); //[pre-mono]~ ERROR unvalidated
    //[pre-mono]~^ NOTE does not know
    T::validated(); // ok
    T::default_is_validated(); // ok
}

#[ferrocene::prevalidated] //~ NOTE marked
fn call(f: impl Fn()) { //~ NOTE validated
    #![warn(ferrocene::possibly_unvalidated)] //~ NOTE lint level
    f(); //~ WARN possibly
    //~^ NOTE does not know
    //[post-mono]~^^ ERROR unvalidated
    //[post-mono]~^^^ NOTE is unvalidated
    //[post-mono]~^^^^ NOTE known_unvalidated
}

#[ferrocene::prevalidated] //[post-mono]~ NOTE marked
pub fn indirect_checks() { //[post-mono]~ NOTE is validated
    call(HasValidations::default_is_validated); // OK
    call(HasValidations::validated); // OK
    call(HasValidations::unvalidated); // OK: even though the trait isn't validated, the impl is
    call(HasValidations::default_unvalidated); //[post-mono]~ NOTE instantiated
}
