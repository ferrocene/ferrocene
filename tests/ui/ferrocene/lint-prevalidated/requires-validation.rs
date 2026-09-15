// `#[ferrocene::requires_validation]` on a trait method requires every implementation of that
// method to be marked `#[ferrocene::prevalidated]`.

#![crate_type = "lib"]
#![deny(ferrocene::unvalidated)]
//~^ NOTE lint level

trait Trait {
    #[ferrocene::requires_validation]
    //~^ NOTE required to be validated here
    fn requires_validation_no_default(&self) -> u8;
    //~^ NOTE all implementations of

    #[ferrocene::prevalidated]
    #[ferrocene::requires_validation]
    //~^ NOTE required to be validated here
    fn requires_validation_yes_default(&self) -> u8 {
        //~^ NOTE all implementations of
        1
    }

    #[ferrocene::requires_validation]
    //~^ NOTE required to be validated here
    const ASSOC_CONST_1: fn() -> u8;
    //~^ NOTE all implementations of

    #[ferrocene::prevalidated]
    #[ferrocene::requires_validation]
    //~^ NOTE required to be validated here
    const ASSOC_CONST_2: fn() -> u8 = || 0;
    //~^ NOTE all implementations of
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

    #[ferrocene::prevalidated]
    const ASSOC_CONST_1: fn() -> u8 = || 1;

    #[ferrocene::prevalidated]
    const ASSOC_CONST_2: fn() -> u8 = || 2;
}

// Inheriting the default body needs no annotation: the default is validated.
struct GoodUsesDefault;

impl Trait for GoodUsesDefault {
    #[ferrocene::prevalidated]
    fn requires_validation_no_default(&self) -> u8 {
        6
    }

    #[ferrocene::prevalidated]
    const ASSOC_CONST_1: fn() -> u8 = || 3;
}

struct Bad;

impl Trait for Bad {
    fn requires_validation_no_default(&self) -> u8 {
        //~^ ERROR unvalidated method implements a trait item that requires validation
        //~| NOTE this implementation is unvalidated
        //~| HELP add `#[ferrocene::prevalidated]`
        4
    }

    fn requires_validation_yes_default(&self) -> u8 {
        //~^ ERROR unvalidated method implements a trait item that requires validation
        //~| NOTE this implementation is unvalidated
        //~| HELP add `#[ferrocene::prevalidated]`
        5
    }

    const ASSOC_CONST_1: fn() -> u8 = || 4;
    //~^ ERROR unvalidated
    //~| NOTE this implementation is unvalidated
    //~| HELP add `#[ferrocene::prevalidated]` to this implementation

    const ASSOC_CONST_2: fn() -> u8 = || 5;
    //~^ ERROR unvalidated
    //~| NOTE this implementation is unvalidated
    //~| HELP add `#[ferrocene::prevalidated]` to this implementation
}
