// `#[ferrocene::requires_validation]` on a trait method requires every implementation of that
// method to be marked `#[ferrocene::prevalidated]`.

#![crate_type = "lib"]
#![deny(ferrocene::unvalidated)]
//~^ NOTE lint level

trait Trait {
    #[ferrocene::requires_validation]
    //~^ NOTE required to be validated here
    //~^^ NOTE required to be validated here
    fn requires_validation_no_default(&self) -> u8;
    //~^ NOTE all implementations of
    //~^^ NOTE all implementations of

    #[ferrocene::prevalidated]
    #[ferrocene::requires_validation]
    //~^ NOTE required to be validated here
    fn requires_validation_yes_default(&self) -> u8 {
        //~^ NOTE all implementations of
        1
    }
}

struct Good;

impl Trait for Good {
    #[ferrocene::prevalidated]
    fn requires_validation_no_default(&self) -> u8 {
        2
    }

    #[ferrocene::prevalidated]
    fn requires_validation_yes_default(&self) -> u8 {
        3
    }
}

struct Bad;

impl Trait for Bad {
    fn requires_validation_no_default(&self) -> u8 {
        //~^ ERROR unvalidated method implements a trait method that requires validation
        //~| NOTE this implementation is unvalidated
        //~| HELP add `#[ferrocene::prevalidated]`
        4
    }

    fn requires_validation_yes_default(&self) -> u8 {
        //~^ ERROR unvalidated method implements a trait method that requires validation
        //~| NOTE this implementation is unvalidated
        //~| HELP add `#[ferrocene::prevalidated]`
        5
    }
}

// Inheriting the default body needs no annotation: the default is validated.
struct UsesDefault;

impl Trait for UsesDefault {
    fn requires_validation_no_default(&self) -> u8 {
        //~^ ERROR unvalidated method implements a trait method that requires validation
        //~| NOTE this implementation is unvalidated
        //~| HELP add `#[ferrocene::prevalidated]`
        6
    }
}
