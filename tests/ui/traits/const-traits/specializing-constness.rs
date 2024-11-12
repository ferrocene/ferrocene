#![feature(const_trait_impl, min_specialization, rustc_attrs)]

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
//FIXME(const_trait_impl) ~| ERROR: missing `~const` qualifier
    fn a() -> u32 {
        3
    }
}

fn main() {}
