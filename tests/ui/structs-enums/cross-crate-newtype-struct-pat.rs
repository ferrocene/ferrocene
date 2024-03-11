//@ run-pass
//@ aux-build:newtype_struct_xc.rs


extern crate newtype_struct_xc;

pub fn main() {
    let x = newtype_struct_xc::Au(21);
    match x {
        newtype_struct_xc::Au(n) => assert_eq!(n, 21)
    }
}

// ferrocene-annotations: fls_maw4u1o8q37u
// Crates
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
