//@ edition:2018
#![warn(rust_2021_prelude_collisions)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod m {
    pub trait TryIntoU32 {
        fn try_into(self) -> Result<u32, ()>;
    }

    impl TryIntoU32 for u8 {
        fn try_into(self) -> Result<u32, ()> {
            Ok(self as u32)
        }
    }

    pub trait AnotherTrick {}
}

mod d {
    use crate::m::AnotherTrick as TryIntoU32;
    use crate::m::*;

    fn main() {
        // Here, `TryIntoU32` is imported but shadowed, but in that case we don't permit its methods
        // to be available.
        let _: u32 = 3u8.try_into().unwrap();
        //~^ ERROR no method named `try_into` found for type `u8` in the current scope
    }
}

fn main() {}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
