// Check for unused crate dep, deny, expect failure

//@ edition:2018
//@ compile-flags: -Dunused-crate-dependencies
//@ aux-crate:bar=bar.rs

fn main() {}
<<<<<<< HEAD
//~^ ERROR external crate `bar` unused in

// ferrocene-annotations: um_rustc_D
=======
//~^ ERROR extern crate `bar` is unused in
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
