//@ check-pass

pub extern crate self as name;
pub use name::name as bug;

fn main() {}

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
