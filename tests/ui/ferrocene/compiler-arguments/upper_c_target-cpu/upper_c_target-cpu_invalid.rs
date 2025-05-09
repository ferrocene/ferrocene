//@ compile-flags: -Ctarget-cpu=invalid
//@ failure-status: 101
//@ build-fail
//@ ignore-cross-compile
//@ ignore-aarch64 This test does not produce the final line of the stderr about 64 bit code
//~? RAW not a recognized processor for this target
//~? WARN target feature `x87` must be enabled

// Note: we have different behavior when cross-compiling (compared to when not),
// so this test is disabled when cross-compiling.

fn main() {}

// ferrocene-annotations: um_rustc_C_target_cpu
