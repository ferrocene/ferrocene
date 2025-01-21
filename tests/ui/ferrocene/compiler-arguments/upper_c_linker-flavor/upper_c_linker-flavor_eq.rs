//@ revisions: gcc lld
//
//@ [gcc] compile-flags: -Clinker-flavor=gcc
//@ [gcc] ignore-thumb
//
//@ [lld] compile-flags: -Clinker=rust-lld -Clinker-flavor=ld.lld
//@ [lld] only-thumb
//
//@ build-pass

fn main() {}

// ferrocene-annotations: um_rustc_C_linker_flavor
