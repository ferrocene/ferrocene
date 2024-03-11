//@ aux-build:on_structs_and_enums_xc.rs

extern crate on_structs_and_enums_xc;

use on_structs_and_enums_xc::{Bar, Foo, Trait};

fn main() {
    let foo = Foo {
        x: 3
    //~^ ERROR E0277
    };
    let bar: Bar<f64> = return;
    //~^ ERROR E0277
    let _ = bar;
}

// ferrocene-annotations: fls_maw4u1o8q37u
// Crates
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
//
// ferrocene-annotations: fls_i7g2n7hfg3ch
// Generic Conformance
