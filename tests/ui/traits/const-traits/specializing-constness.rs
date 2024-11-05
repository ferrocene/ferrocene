#![feature(const_trait_impl, effects, min_specialization, rustc_attrs)] //~ WARN the feature `effects` is incomplete

#[rustc_specialization_trait]
#[const_trait]
pub trait Sup {}

impl const Sup for () {}

#[const_trait]
pub trait A {
    fn a() -> u32;
}

#[const_trait]
pub trait Spec {}

impl<T: ~const Spec> const A for T {
    default fn a() -> u32 {
        2
    }
}

impl<T: Spec + Sup> A for T {
//~^ ERROR: cannot specialize
//FIXME(effects) ~| ERROR: missing `~const` qualifier
    fn a() -> u32 {
        3
    }
}

fn main() {}
