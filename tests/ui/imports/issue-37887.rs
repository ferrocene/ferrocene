//@ edition: 2015
fn main() {
    extern crate test; //~ ERROR use of unstable
    use test::*; //~ ERROR unresolved import
}

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
