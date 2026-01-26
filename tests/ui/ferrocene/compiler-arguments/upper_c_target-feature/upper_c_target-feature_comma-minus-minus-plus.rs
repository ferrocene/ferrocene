//@ build-pass
//@ compile-flags: -Ctarget-feature=-foo,-bar,+baz
//
// the warning about the unknown target feature is printed repeatedly to stderr on _some_ targets;
// omit checking stderr to avoid failing due to the inconsistent diagnostics (upstream bug)
// note that the JSON version of the diagnostic, which is bug-free, is still checked
//@ dont-check-compiler-stderr
//~? RAW not a recognized feature for this target
//~? WARN unknown and unstable feature specified
//~? WARN unknown and unstable feature specified
//~? WARN unknown and unstable feature specified

fn main() {}

// ferrocene-annotations: um_rustc_C_target_feature
