//@ error-pattern:can't find crate for `fake_crate`

extern crate fake_crate as extra;

fn main() { }

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
