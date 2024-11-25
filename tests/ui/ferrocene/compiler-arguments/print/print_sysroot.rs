//@ check-pass
//@ compile-flags: --print sysroot
//
//@ normalize-stdout-test: "stage[012]" -> "$$STAGE"
//@ normalize-stdout-test: "ci-rustc-sysroot" -> "$$STAGE"
//
// If this test fails on a new platform, add a new normalization annotation:
//@ normalize-stdout-test: "x86_64-unknown-linux-gnu" -> "$$TARGET"
//@ normalize-stdout-test: "x86_64-pc-windows-msvc" -> "$$TARGET"
//@ normalize-stdout-test: "aarch64-unknown-linux-gnu" -> "$$TARGET"
//@ normalize-stdout-test: "aarch64-apple-darwin" -> "$$TARGET"

fn main() {}

// ferrocene-annotations: um_rustc_print
