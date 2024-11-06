// When linker is specified, but invalid, we have a linker error, so we get it
// on the build step.
//
//@ build-fail
//@ compile-flags: -Clinker=invalid
//@ ignore-windows: The output on this platform differs

fn main() {}

// ferrocene-annotations: um_rustc_C_linker
