//@ check-fail
//@ compile-flags: --crate-type=bin

fn foo() {} //~ ERROR `main` function not found in crate

// ferrocene-annotations: um_rustc_crate_type
