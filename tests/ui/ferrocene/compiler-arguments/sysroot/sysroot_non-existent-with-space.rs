// Test when --sysroot set the path to a non-existent sysroot
//
//@ check-fail
//@ ignore-cross-compile The error message is different when cross-compiling
//@ only-x86_64 Tested stderr is x86_64-linux-gnu specific
//@ only-linux Tested stderr is x86_64-linux-gnu specific
//@ compile-flags: --sysroot /non/existent/sysroot
//~^^^^^ ERROR can't find crate for `std`

fn main() {}

// ferrocene-annotations: um_rustc_sysroot
