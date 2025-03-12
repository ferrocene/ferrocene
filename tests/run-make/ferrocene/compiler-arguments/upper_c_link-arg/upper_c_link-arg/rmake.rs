use std::path::Path;

use run_make_support::{rustc, target};

fn main() {
    rustc()
        .edition("2021")
        .target(target())
        .link_arg("-lfoo")
        .print("link-args")
        .input("main.rs")
        .run_unchecked()
        .assert_stdout_contains("lfoo");
}

// ferrocene-annotations: um_rustc_C_link_arg
