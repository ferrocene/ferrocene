//@ run-pass
//@ aux-build:struct_variant_xc_aux.rs

extern crate struct_variant_xc_aux;

use struct_variant_xc_aux::Enum::StructVariant;

pub fn main() {
    let _ = StructVariant { arg: 1 };
}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_maw4u1o8q37u
// Crates
//
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
