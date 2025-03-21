//@ aux-build:extern-statics.rs

extern crate extern_statics;
use extern_statics::*;

extern "C" {
    static mut B: u8;
}

fn main() {
    let b = B; //~ ERROR use of mutable static is unsafe
    let rb = &B; //~ ERROR use of mutable static is unsafe
    let xb = XB; //~ ERROR use of mutable static is unsafe
    let xrb = &XB; //~ ERROR use of mutable static is unsafe
}

// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
//
// ferrocene-annotations: fls_s4yt19sptl7d
// External Statics
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
