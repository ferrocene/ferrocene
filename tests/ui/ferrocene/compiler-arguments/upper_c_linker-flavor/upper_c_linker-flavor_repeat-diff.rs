//@ build-pass
//@ only-linux
//@ ignore-thumb ferrocenecoretest register as "linux" but do not use gcc as a linker
//@ compile-flags: -Clinker-flavor=msvc -Clinker-flavor=gcc

// When repeated, the last one overrides the previous.
// Here the build must pass when building for linux, because the first linker
// flavor set to msvc is override by the next flavor set to gcc.

fn main() {}

// ferrocene-annotations: um_rustc_C_linker_flavor
