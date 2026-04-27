//@ revisions: build-script regular-crate
//@[build-script] check-pass
//@[regular-crate] check-fail

//@[build-script] compile-flags: --crate-name=build_script_build
//@[build-script] rustc-env: CARGO_CRATE_NAME=build_script_build

#![deny(ferrocene::unvalidated)]

fn unvalidated() {}
fn main() { unvalidated(); }
//[regular-crate]~^ ERROR unvalidated
