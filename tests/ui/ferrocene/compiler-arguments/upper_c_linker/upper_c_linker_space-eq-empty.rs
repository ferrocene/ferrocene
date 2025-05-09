// When linker is specified, but empty, we have a linker error, so we get it
// on the build step.
//
//@ build-fail
//@ compile-flags: -C linker=
//~? ERROR couldn't extract file stem from specified linker

fn main() {}

// ferrocene-annotations: um_rustc_C_linker
