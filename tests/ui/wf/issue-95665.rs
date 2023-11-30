// Regression test for the ICE described in #95665.
// Ensure that the expected error is output (and thus that there is no ICE)

pub trait Trait: {}

pub struct Struct<T: Trait> {
    member: T,
}

// uncomment and bug goes away
// impl Trait for u8 {}

extern "C" {
    static VAR: Struct<u8>;
                //~^ 14:17: 14:27: the trait bound `u8: Trait` is not satisfied [E0277]
}

fn main() {}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
//
// ferrocene-annotations: fls_s4yt19sptl7d
// External Statics
