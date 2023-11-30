// compile-flags: --cfg a{
// error-pattern: invalid `--cfg` argument: `a{` (expected `key` or `key="value"`)
fn main() {}

// ferrocene-annotations: um_rustc_cfg
