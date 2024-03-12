//@ compile-flags: -Ctarget-cpu=invalid
//@ failure-status: 101
//@ build-fail
//@ ignore-cross-compile

// Note: we have differente behaviors when cross compiling and when not cross
// compiling. So this test is disabled when cross-compiling

fn main() {}

// ferrocene-annotations: um_rustc_C_target_cpu
