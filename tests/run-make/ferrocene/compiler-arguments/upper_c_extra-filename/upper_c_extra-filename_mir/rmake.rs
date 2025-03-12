use std::path::Path;

use run_make_support::{rfs, rustc, target};

// Suffix is added before the extension
fn main() {
    rustc()
        .edition("2021")
        .target(target())
        .emit("mir")
        .input("main.rs")
        .extra_filename("foo")
        .run();

    assert!(Path::new("mainfoo.mir").is_file())
}

// ferrocene-annotations: um_rustc_C_extra_filename
