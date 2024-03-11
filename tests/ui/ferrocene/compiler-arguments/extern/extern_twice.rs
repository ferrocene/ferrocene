//@ check-pass
//@ aux-build:some_crate.rs
//@ compile-flags:--extern some_crate --extern some_crate
//@ edition:2018

fn main() {
    ::some_crate::hello();
}

// ferrocene-annotations: um_rustc_extern
