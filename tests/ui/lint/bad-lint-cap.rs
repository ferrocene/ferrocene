//@ compile-flags: --cap-lints test

fn main() {}

//~? ERROR unknown lint level: `test`

// ferrocene-annotations: um_rustc_cap_lints
