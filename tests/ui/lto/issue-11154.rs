//@ build-fail
//@ compile-flags: -C lto -C prefer-dynamic

fn main() {}

//~? ERROR cannot prefer dynamic linking when performing LTO

// ferrocene-annotations: um_rustc_C_prefer_dynamic
