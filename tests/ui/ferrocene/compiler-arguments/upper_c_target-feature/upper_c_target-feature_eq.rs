//@ build-pass
//@ compile-flags: -Ctarget-feature=+foo
//
//~? RAW not a recognized feature for this target
//~? WARN unknown and unstable feature specified

fn main() {}

// ferrocene-annotations: um_rustc_C_target_feature
