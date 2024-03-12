//@ compile-flags: --cap-lints deny

#![warn(unused)]
#![deny(warnings)]

use std::option; //~ ERROR

fn main() {}

// ferrocene-annotations: um_rustc_cap_lints
