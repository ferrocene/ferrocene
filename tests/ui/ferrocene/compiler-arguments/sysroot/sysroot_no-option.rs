//~ ERROR can't find crate for `std`

// Test when --sysroot set the path to a empty sysroot. sysroot should be the
// current path.
//
//@ check-fail
//@ needs-llvm-components: x86
//@ compile-flags: --sysroot= --target=x86_64-unknown-linux-gnu

fn main() {}

// ferrocene-annotations: um_rustc_sysroot
