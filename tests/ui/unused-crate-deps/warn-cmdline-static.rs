// Check for unused crate dep, no path

//@ edition:2018
//@ check-pass
//@ compile-flags: -Wunused-crate-dependencies
//@ aux-crate:bar=bar.rs
//@ no-prefer-dynamic

fn main() {}
<<<<<<< HEAD
//~^ WARNING external crate `bar` unused in

// ferrocene-annotations: um_rustc_W
=======
//~^ WARNING extern crate `bar` is unused in
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
