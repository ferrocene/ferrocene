//@ build-pass
//@ compile-flags: -Ctarget-feature=invalid

// Each feature should be prefixed with a + to enable it or - to disable it.
// Not having + or - prefix is considered invalid, but the build still passes.

fn main() {}

// ferrocene-annotations: um_rustc_C_target_feature
