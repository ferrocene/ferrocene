//@ run-pass
//@ compile-flags: -Z thinlto -C codegen-units=2

#[global_allocator]
static A: std::alloc::System = std::alloc::System;

fn main() {}

// ferrocene-annotations: um_rustc_C_codegen_units
