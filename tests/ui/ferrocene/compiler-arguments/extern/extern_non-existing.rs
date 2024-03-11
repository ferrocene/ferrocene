//@ check-fail
//@ aux-build:some_crate.rs
//@ compile-flags:--extern some_crate=non_existing
//@ edition:2018

fn main() {
    ::some_crate::hello(); //~ ERROR extern location for some_crate does not exist: non_existing
}

// ferrocene-annotations: um_rustc_extern
