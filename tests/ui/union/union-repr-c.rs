#![allow(unused)]
#![deny(improper_ctypes)]

#[repr(C)]
union U {
    a: u8,
}

union W {
    a: u8,
}

extern "C" {
    static FOREIGN1: U; // OK
    static FOREIGN2: W; //~ ERROR `extern` block uses type `W`
}

fn main() {}

// ferrocene-annotations: fls_aibb2quva4mn
// Attribute repr
//
// ferrocene-annotations: fls_cmq8ogs84ivh
// Union Type Representation
//
// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
//
// ferrocene-annotations: fls_s4yt19sptl7d
// External Statics
