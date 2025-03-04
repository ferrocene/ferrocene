//@ check-pass
//@ compile-flags: --print sysroot
//
//@ normalize-stdout: "stage[012]" -> "$$STAGE"
//@ normalize-stdout: "ci-rustc-sysroot" -> "$$STAGE"
//
// If this test fails on a new platform, add a new normalization annotation:
//@ normalize-stdout: "x86_64-unknown-linux-gnu" -> "$$TARGET"
//@ normalize-stdout: "x86_64-pc-windows-msvc" -> "$$TARGET"
//@ normalize-stdout: "aarch64-unknown-linux-gnu" -> "$$TARGET"
//@ normalize-stdout: "aarch64-apple-darwin" -> "$$TARGET"

fn main() {}

// ferrocene-annotations: um_rustc_print
