fn main() {
    extern crate libc; //~ ERROR use of unstable
    use libc::*; //~ ERROR unresolved import
}

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
