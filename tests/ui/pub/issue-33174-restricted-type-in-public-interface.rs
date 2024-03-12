//@ check-pass

#![allow(non_camel_case_types)] // genus is always capitalized

pub(crate) struct Snail;

mod sea {
    pub(super) struct Turtle;
}

struct Tortoise;

pub struct Shell<T> {
    pub(crate) creature: T,
}

pub type Helix_pomatia = Shell<Snail>;
//~^ WARNING type `Snail` is more private than the item `Helix_pomatia`
pub type Dermochelys_coriacea = Shell<sea::Turtle>;
//~^ WARNING type `Turtle` is more private than the item `Dermochelys_coriacea`
pub type Testudo_graeca = Shell<Tortoise>;
//~^ WARNING type `Tortoise` is more private than the item `Testudo_graeca`

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
