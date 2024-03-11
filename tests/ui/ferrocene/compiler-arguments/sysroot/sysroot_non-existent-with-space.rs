//@ Test when --sysroot set the path to a non-existent sysroot
//
//@ check-fail
//@ ignore-cross-compile The error message is different when cross-compiling
//@ compile-flags: --sysroot /non/existent/sysroot
//~^^^^^ ERROR can't find crate for `std`

fn main() {}

// ferrocene-annotations: um_rustc_sysroot
