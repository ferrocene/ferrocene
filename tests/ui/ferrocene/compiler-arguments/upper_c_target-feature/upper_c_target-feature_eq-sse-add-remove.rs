//@ build-pass
//@ compile-flags: -Ctarget-feature=+sse,-sse
//@ only-x86_64-unknown-linux-gnu

fn main() {}

// ferrocene-annotations: um_rustc_C_target_feature
