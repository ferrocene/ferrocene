//@ check-pass

#![allow(stable_features)]

#![feature(rust1)]
#![feature(rust1)] //~ WARN the feature `rust1` has already been enabled

#![feature(if_let)]
#![feature(if_let)] //~ WARN the feature `if_let` has already been enabled

fn main() {}
