// Test when --sysroot set the path to a non-existent sysroot
//
//@ check-fail
//@ compile-flags: --sysroot=/non/existent/sysroot
//@ only-x86_64 Tested stderr is x86_64-linux-gnu specific
//@ only-linux Tested stderr is x86_64-linux-gnu specific
//@ ignore-cross-compile The error message is different when cross-compiling
//~^^^^^^^ ERROR can't find crate for `std`

fn main() {}

// ferrocene-annotations: um_rustc_sysroot
