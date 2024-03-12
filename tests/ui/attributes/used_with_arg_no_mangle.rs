//@ check-pass

#![feature(used_with_arg)]

#[used(linker)]
#[no_mangle] // accidentally detected as `used(compiler)`
pub static GLOB: usize = 0;

fn main() {}

// ferrocene-annotations: fls_mvd7nz8k3wcy
// Attribute no_mangle
//
// ferrocene-annotations: fls_7skf24auayqy
// Attribute used
