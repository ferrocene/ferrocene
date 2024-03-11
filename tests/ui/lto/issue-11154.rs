//@ build-fail
//@ compile-flags: -C lto -C prefer-dynamic

//@ error-pattern: cannot prefer dynamic linking

fn main() {}

// ferrocene-annotations: um_rustc_C_prefer_dynamic
