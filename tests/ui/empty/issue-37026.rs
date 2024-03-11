//@ aux-build:empty-struct.rs

extern crate empty_struct;

fn main() {
    let empty_struct::XEmpty2 = (); //~ ERROR mismatched types
    let empty_struct::XEmpty6(..) = (); //~ ERROR mismatched types
}

// ferrocene-annotations: fls_uloyjbaso8pz
// Path Patterns
// ferrocene-annotations: fls_d44aflefat88
// Path Pattern Matching
// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
// ferrocene-annotations: fls_eexupzdsu7f
// Tuple Struct Pattern Matching
