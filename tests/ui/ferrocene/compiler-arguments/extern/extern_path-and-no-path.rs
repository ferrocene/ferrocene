//@ check-fail
//@ aux-build:some_crate.rs
//@ compile-flags:--extern some_crate=invalid --extern some_crate
//@ edition:2018

fn main() {
    ::some_crate::hello(); //~ ERROR extern location for some_crate does not exist: invalid
}

// ferrocene-annotations: um_rustc_extern
