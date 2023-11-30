#![allow(non_camel_case_types)] // genus is always capitalized

pub(crate) struct Snail;
//~^ NOTE `Snail` declared as private

mod sea {
    pub(super) struct Turtle;
    //~^ NOTE `Turtle` declared as crate-private
}

struct Tortoise;
//~^ NOTE `Tortoise` declared as private

pub struct Shell<T> {
    pub(crate) creature: T,
}

pub type Helix_pomatia = Shell<Snail>;
//~^ ERROR private type `Snail` in public interface
//~| NOTE can't leak private type
pub type Dermochelys_coriacea = Shell<sea::Turtle>;
//~^ ERROR crate-private type `Turtle` in public interface
//~| NOTE can't leak crate-private type
pub type Testudo_graeca = Shell<Tortoise>;
//~^ ERROR private type `Tortoise` in public interface
//~| NOTE can't leak private type

fn main() {}

// ferrocene-annotations: fls_kgvleup5mdhq
// Type Aliases
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
