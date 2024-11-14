//@ compile-flags: -Cprefer-dynamic

#![crate_type = "dylib"]
pub fn bar() {}

// ferrocene-annotations: um_rustc_C_prefer_dynamic
