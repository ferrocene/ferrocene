//~ ERROR can't find crate for `std`

// Test when --sysroot set the path to a non-existent sysroot
//
//@ check-fail
//@ needs-llvm-components: x86
//@ compile-flags: --sysroot /non/existent/sysroot --target=x86_64-unknown-linux-gnu

fn main() {}

// ferrocene-annotations: um_rustc_sysroot
