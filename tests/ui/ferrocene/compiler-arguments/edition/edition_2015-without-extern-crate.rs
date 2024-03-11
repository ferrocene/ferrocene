//@ check-fail
//@ aux-build:some_crate.rs
//@ compile-flags:--extern some_crate --edition 2015

fn main() {
    ::some_crate::hello(); //~ ERROR [E0433]
}

// ferrocene-annotations: um_rustc_edition
