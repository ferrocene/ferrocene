// `prevalidated` and `requires_validation` are rejected in positions where they have no meaning,
// or are invalid.
//
// None of these items are called or implemented: the errors come from the definitions alone.

#![crate_type = "lib"]

trait Trait {
    #[ferrocene::prevalidated]
    //~^ ERROR `#[ferrocene::prevalidated]` cannot be applied to a method without a default body
    //~| HELP use `#[ferrocene::requires_validation]`
    fn prevalidated_no_body(&self);
    //~^ NOTE this method has no body to validate

    #[ferrocene::requires_validation]
    fn requires_validation_no_body(&self);

    #[ferrocene::prevalidated]
    #[ferrocene::requires_validation]
    fn prevalidated_and_requires_validation_and_body(&self) {}

    #[ferrocene::requires_validation]
    //~^ ERROR `#[ferrocene::requires_validation]` on a method with a default body requires `#[ferrocene::prevalidated]` as well
    //~| HELP add `#[ferrocene::prevalidated]` to validate the default body
    fn requires_validation_yes_body(&self) {}
    //~^ NOTE this default body is not validated

    #[ferrocene::requires_validation]
    //~^ ERROR associated constants that are no function pointer cannot be marked `#[ferrocene::requires_validation]`
    //~| HELP remove `#[ferrocene::requires_validation]`
    const ASSOCIATED_CONST_NO_FN_PTR: u8;
    //~^ NOTE this const is marked `#[ferrocene::requires_validation]`, but is no function pointer

    #[ferrocene::requires_validation]
    //~^ ERROR `#[ferrocene::requires_validation]` on an associated constant with a default value requires `#[ferrocene::prevalidated]` as well
    //~| HELP add `#[ferrocene::prevalidated]` to validate the default value
    const ASSOCIATED_CONST_YES_FN_PTR: fn() -> u8 = || 0;
    //~^ NOTE this default value is not validated, but implementations inherit it

    #[ferrocene::prevalidated]
    #[ferrocene::requires_validation]
    const REQUIRES_VALIDATION_ASSOC_CONSTANT2: fn() -> u8 = || 1;
}

#[ferrocene::requires_validation]
//~^ ERROR `#[ferrocene::requires_validation]` cannot be applied to a function
//~| NOTE `#[ferrocene::requires_validation]` can only be applied to associated trait items
fn free() {}

struct S;

impl S {
    #[ferrocene::requires_validation]
    //~^ ERROR `#[ferrocene::requires_validation]` cannot be applied to a method
    //~| NOTE `#[ferrocene::requires_validation]` can only be applied to associated trait items
    fn inherent(&self) {}

    #[ferrocene::requires_validation]
    //~^ ERROR: `#[ferrocene::requires_validation]` cannot be applied to an associated constant
    //~| NOTE: `#[ferrocene::requires_validation]` can only be applied to associated trait items
    const REQUIRES_VALIDATION_ASSOC_CONSTANT3: fn() -> u8 = || 3;
}
