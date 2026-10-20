//@ build-pass

#![crate_type = "lib"]
#![warn(ferrocene::unvalidated)]

#[ferrocene::prevalidated]
pub fn usage() {
    uncertified_coerce_with_certified_instantiation(&certified_fun); //~ WARN unvalidated
    //~^ WARN unvalidated
    uncertified_coerce_with_uncertified_instantiation(&uncertified_fun); //~ WARN unvalidated
    //~^ WARN unvalidated

    certified_coerce_with_certified_instantiation(&certified_fun); // OK
    certified_coerce_with_uncertified_instantiation(&uncertified_fun); // warning at definition site

    certified_coerce_with_both_instantiations(&certified_fun); // OK
    certified_coerce_with_both_instantiations(&uncertified_fun); // warning at definition site

    // these two produce the same diagnostics at different sites
    certified_coerce_with_closure_literal(&|| {});
    let _: &dyn Fn() = &|| {}; //~ WARN possibly calls an unvalidated method

    certified_call(&|| {}); // OK
}

fn uncertified_coerce_with_certified_instantiation<F>(callback: &F) -> &dyn Fn()
where
    F: Fn(),
{
    callback
}

fn uncertified_coerce_with_uncertified_instantiation<F>(callback: &F) -> &dyn Fn()
//~^ WARN possibly calls an unvalidated function
where
    F: Fn(),
{
    callback
}

#[ferrocene::prevalidated]
fn certified_coerce_with_uncertified_instantiation<F>(callback: &F) -> &dyn Fn()
//~^ WARN possibly calls an unvalidated function
where
    F: Fn(),
{
    callback
}

#[ferrocene::prevalidated]
fn certified_coerce_with_certified_instantiation<F>(callback: &F) -> &dyn Fn()
where
    F: Fn(),
{
    callback
}

#[ferrocene::prevalidated]
fn certified_coerce_with_both_instantiations<F>(callback: &F) -> &dyn Fn()
//~^ WARN possibly calls an unvalidated function
where
    F: Fn(),
{
    callback
}

#[ferrocene::prevalidated]
fn certified_coerce_with_closure_literal<F>(callback: &F) -> &dyn Fn()
//~^ WARN possibly calls an unvalidated method
where
    F: Fn(),
{
    callback
}

#[ferrocene::prevalidated]
fn certified_call<F>(callback: &F)
where
    F: Fn(),
{
    callback();
}

#[ferrocene::prevalidated]
fn certified_fun() {}

fn uncertified_fun() {}
