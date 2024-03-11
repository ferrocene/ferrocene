//@ compile-flags: -l link-arg:+bundle=arg -Z unstable-options
//@ error-pattern: linking modifier `bundle` is only compatible with `static` linking kind

fn main() {}

// ferrocene-annotations: um_rustc_l
