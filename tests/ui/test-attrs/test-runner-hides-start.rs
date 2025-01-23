<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
//@ run-pass
//@ compile-flags: --test

#![feature(start)]

#[start]
fn start(_: isize, _: *const *const u8) -> isize { panic!(); }

// ferrocene-annotations: um_rustc_test
