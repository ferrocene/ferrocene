//@ run-pass
//@ aux-build:extern_mod_ordering_lib.rs


extern crate extern_mod_ordering_lib;

use extern_mod_ordering_lib::extern_mod_ordering_lib as the_lib;

pub fn main() {
    the_lib::f();
}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
// ferrocene-annotations: fls_gklst7joeo33
// External Crates
