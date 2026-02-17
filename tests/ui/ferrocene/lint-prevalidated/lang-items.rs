// Don't use regular core, so this is independent of any `ferrocene::prevalidated` attributes we
// ship. This variant uses lang items from minicore. Avoid this and use `lint-uncertified-trait-ops`
// where possible.

//@ add-minicore
//@ check-pass

#![crate_type = "lib"]
#![feature(no_core)]
#![no_std]
#![no_core]
#![deny(ferrocene::uncertified)]

extern crate minicore;
use minicore::*;
use Result::*;

#[ferrocene::prevalidated]
fn ops() {
    let f: fn() = || (); f(); // ok
    (|| ())(); // ok
}
