// Test when --sysroot set the path to a empty sysroot. sysroot should be the
// current path.
//
//@ check-fail
//@ ignore-cross-compile The error message is different when cross-compiling
//@ compile-flags: --sysroot=  --target=x86_64-unknown-linux-gnu
//~^^^^^^ ERROR can't find crate for `std`

fn main() {}

// ferrocene-annotations: um_rustc_sysroot
