//@ run-pass

//@ compile-flags:-C panic=abort
//@ no-prefer-dynamic
//@ ignore-macos

#![feature(panic_abort)]

extern crate panic_abort;

fn main() {}

// ferrocene-annotations: um_rustc_C_panic
