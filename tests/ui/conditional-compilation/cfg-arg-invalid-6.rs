//@ compile-flags: --cfg a{

fn main() {}

//~? ERROR invalid `--cfg` argument: `a{` (expected `key` or `key="value"`)

// ferrocene-annotations: um_rustc_cfg
