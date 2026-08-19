// `prevalidated` and `requires_validation` are rejected in positions where they have no meaning,
// or are invalid.
//
// None of these items are called or implemented: the errors come from the definitions alone.

#![crate_type = "lib"]

trait Trait {
    #[ferrocene::prevalidated]
    //~^ ERROR `#[ferrocene::prevalidated]` cannot be applied to a trait method without a default body
    //~| HELP use `#[ferrocene::requires_validation]`
    fn prevalidated_no_body(&self);
    //~^ NOTE this method has no body to validate

    #[ferrocene::requires_validation]
    fn requires_validation_no_body(&self);

    #[ferrocene::prevalidated]
    #[ferrocene::requires_validation]
    fn prevalidated_and_requires_validation_and_body(&self) {}

    #[ferrocene::requires_validation]
    //~^ ERROR `#[ferrocene::requires_validation]` on a trait method with a default body requires `#[ferrocene::prevalidated]` as well
    //~| HELP add `#[ferrocene::prevalidated]` to validate the default body
    fn requires_validation_yes_body(&self) {}
    //~^ NOTE this default body is not validated

    #[ferrocene::requires_validation]
    const REQUIRES_VALIDATION_ASSOC_CONSTANT: u8;

    #[ferrocene::prevalidated]
    #[ferrocene::requires_validation]
    const REQUIRES_VALIDATION_ASSOC_CONSTANT2: u8 = 1;
}

#[ferrocene::requires_validation]
//~^ ERROR `#[ferrocene::requires_validation]` cannot be applied to a function
//~| NOTE only trait methods or associated constants containing a fn pointer can be marked
//~| HELP use `#[ferrocene::prevalidated]`
fn free() {}
//~^ NOTE not a trait method

struct S;

impl S {
    #[ferrocene::requires_validation]
    //~^ ERROR `#[ferrocene::requires_validation]` cannot be applied to a method
    //~| NOTE only trait methods or associated constants containing a fn pointer can be marked
    //~| HELP use `#[ferrocene::prevalidated]`
    fn inherent(&self) {}
    //~^ NOTE not a trait method

    #[ferrocene::requires_validation]
    //~^ ERROR: `#[ferrocene::requires_validation]` cannot be applied to an associated constant
    //~| NOTE: only trait methods or associated constants containing a fn pointer can be marked
    //~| HELP: use `#[ferrocene::prevalidated]`
    const REQUIRES_VALIDATION_ASSOC_CONSTANT3: u8 = 0;
    //~^ NOTE: not a trait method
}
