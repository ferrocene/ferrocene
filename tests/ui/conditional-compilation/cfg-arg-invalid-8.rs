//@ compile-flags: --cfg )

fn main() {}

//~? ERROR invalid `--cfg` argument: `)` (expected `key` or `key="value"`)

// ferrocene-annotations: um_rustc_cfg
