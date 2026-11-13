// This checks that the forbid attribute checking is ignored when the forbidden
// lint is capped.

//@ check-pass
//@ compile-flags: --cap-lints=warn -Fwarnings

#![forbid(warnings)]
#![allow(unused)]

#[allow(unused)]
mod bar {
    fn bar() {}
}

fn main() {}

// ferrocene-annotations: um_rustc_F
// ferrocene-annotations: um_rustc_cap_lints
