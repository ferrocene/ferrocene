//@ build-pass
//@ compile-flags: -Ctarget-feature=-foo,-bar,+baz
//
//@ When an invalid target feature is provided, LLVM outputs multiple copies of the warning.
//@ Unfortunately, the number of warnings depends on how many target features are provided, so this
//@ test would fail when executed across tests with different amounts of target features. Comparing
//@ output by subset avoids the problem.
//@ compare-output-lines-by-subset

fn main() {}

// ferrocene-annotations: um_rustc_C_target_feature
