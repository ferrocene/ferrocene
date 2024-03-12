// Multiple debuginfo arguments are passed with different options.
// The last one overrides the previous.
//
//@ check-pass
//@ compile-flags: -Cdebuginfo=1 -Cdebuginfo=0

fn main() {}

// ferrocene-annotations: um_rustc_C_debuginfo
