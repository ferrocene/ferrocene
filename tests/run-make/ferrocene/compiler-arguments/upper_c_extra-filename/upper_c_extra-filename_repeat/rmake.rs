// Every occurrence of -Cextra-filename overrides the previous one.
// Testing only in linux, other OS may add a suffix
//@ only-linux
use std::path::Path;

use run_make_support::{rustc, target};

fn main() {
    rustc()
        .edition("2021")
        .target(target())
        .input("main.rs")
        .extra_filename("foo")
        .extra_filename("bar")
        .run();
    assert!(!Path::new("mainfoo").exists());
    assert!(Path::new("mainbar").is_file())
}

// ferrocene-annotations: um_rustc_C_extra_filename
