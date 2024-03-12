//@ run-pass
//@ compile-flags:--test
// Building as a test runner means that a synthetic main will be run,
// not ours
pub fn main() { panic!(); }

// ferrocene-annotations: um_rustc_test
