//@ check-pass
//@ compile-flags: --print target-libdir
//
//@ normalize-stdout: "stage[012]" -> "$$STAGE"
//@ normalize-stdout: "ci-rustc-sysroot" -> "$$STAGE"
//
// If this test fails on a new platform, add a new normalization annotation:
//@ normalize-stdout: "x86_64-unknown-linux-gnu" -> "$$TARGET"
//@ normalize-stdout: "x86_64-pc-windows-msvc" -> "$$TARGET"
//@ normalize-stdout: "x86_64-pc-nto-qnx710" -> "$$TARGET"
//@ normalize-stdout: "aarch64-unknown-linux-gnu" -> "$$TARGET"
//@ normalize-stdout: "aarch64-unknown-ferrocene.facade" -> "$$TARGET"
//@ normalize-stdout: "thumbv7em-ferrocene.facade-eabihf" -> "$$TARGET"
//@ normalize-stdout: "thumbv7em-ferrocene.facade-eabi" -> "$$TARGET"
//@ normalize-stdout: "aarch64-apple-darwin" -> "$$TARGET"
//@ normalize-stdout: "aarch64-unknown-nto-qnx710" -> "$$TARGET"

fn main() {}

// ferrocene-annotations: um_rustc_print
