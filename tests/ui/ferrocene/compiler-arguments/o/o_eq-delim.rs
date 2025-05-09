//@ check-pass
//@ compile-flags: -o=foo
//@ ferrocene-execute-in-temp
//~? WARN ignoring --out-dir flag due to -o flag

fn main() {}

// ferrocene-annotations: um_rustc_o
