// Derive helpers are resolved successfully inside `cfg_attr`.

//@ check-pass
//@ proc-macro: test-macros.rs

#[macro_use]
extern crate test_macros;

#[derive(Empty)]
#[cfg_attr(all(), empty_helper)]
struct S {
    #[cfg_attr(all(), empty_helper)]
    field: u8,
}

fn main() {}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
//
// ferrocene-annotations: fls_dd9xh3wdjudo
// Attribute cfg_attr
//
// ferrocene-annotations: um_rustc_cfg
