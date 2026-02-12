use std::path::Path;

use run_make_support::{rustc, target};

fn main() {
    rustc()
        .target(target())
        .print("target-features")
        .input("-")
        .run()
        .assert_stdout_contains("Features supported by rustc for this target:")
        .assert_stdout_contains("Code-generation features supported by LLVM for this target:")
        .assert_stdout_contains("Use +feature to enable a feature, or -feature to disable it.");
}

// ferrocene-annotations: um_rustc_print
