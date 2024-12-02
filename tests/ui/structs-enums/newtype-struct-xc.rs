//@ run-pass
//@ aux-build:newtype_struct_xc.rs


extern crate newtype_struct_xc;

pub fn main() {
    let _ = newtype_struct_xc::Au(2);
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
