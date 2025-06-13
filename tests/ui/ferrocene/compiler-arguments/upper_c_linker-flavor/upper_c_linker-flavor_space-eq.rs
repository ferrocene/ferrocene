//@ revisions: gcc lld
//
//@ [gcc] compile-flags: -C linker-flavor=gcc
//@ [gcc] ignore-ferrocene.facade
//
//@ [lld] compile-flags: -Clinker=rust-lld -C linker-flavor=ld.lld
//@ [lld] only-ferrocene.facade
//
//@ build-pass

fn main() {}

// ferrocene-annotations: um_rustc_C_linker_flavor
