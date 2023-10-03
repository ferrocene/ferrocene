pub trait Unsatisfied {}

#[repr(transparent)]
pub struct Bar<T: Unsatisfied>(T);

pub trait Foo {
    type Assoc;
}

extern "C" {
    pub fn lint_me() -> <() as Foo>::Assoc;
    //~^ ERROR: the trait bound `(): Foo` is not satisfied [E0277]

    pub fn lint_me_aswell() -> Bar<u32>;
    //~^ ERROR: the trait bound `u32: Unsatisfied` is not satisfied [E0277]
}

fn main() {}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_aibb2quva4mn
// Attribute repr
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_rjxpof29a3nl
// Struct Type Representation
//
// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
//
// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
