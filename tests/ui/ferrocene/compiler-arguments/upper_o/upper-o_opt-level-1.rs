// -O equals to -C opt-level=2. Here we test what happens when they are both
// set, but with a mismatching optimization level.
//
//@ check-pass
//@ compile-flags: -O -C opt-level=1

fn main() {}

// ferrocene-annotations: um_rustc_O
// ferrocene-annotations: um_rustc_C_opt_level
