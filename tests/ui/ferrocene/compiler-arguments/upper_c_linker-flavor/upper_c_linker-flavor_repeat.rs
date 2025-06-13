//@ revisions: gcc lld
//
//@ [gcc] compile-flags: -Clinker-flavor=gcc -Clinker-flavor=gcc
//@ [gcc] ignore-ferrocene.facade
//
//@ [lld] compile-flags: -Clinker=rust-lld -Clinker-flavor=ld.lld -Clinker-flavor=ld.lld
//@ [lld] only-ferrocene.facade
//
//@ build-pass

fn main() {}

// ferrocene-annotations: um_rustc_C_linker_flavor
