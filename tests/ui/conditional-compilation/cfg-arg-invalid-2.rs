//@ compile-flags: --cfg a{b}

fn main() {}

//~? ERROR invalid `--cfg` argument: `a{b}` (expected `key` or `key="value"`)

// ferrocene-annotations: um_rustc_cfg
