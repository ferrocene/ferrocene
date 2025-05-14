//@ compile-flags: --cfg foo --check-cfg=cfg(foo,bar)

// main is conditionally compiled, but the conditional compilation
// is conditional too!

#[cfg_attr(foo, cfg(bar))]
fn main() { } //~ ERROR `main` function not found in crate `cfg_attr_cfg_2`

// ferrocene-annotations: fls_8jb3sjqamdpu
// Program Entry Point
//
// ferrocene-annotations: fls_dd9xh3wdjudo
// Attribute cfg_attr
//
// ferrocene-annotations: um_rustc_cfg
