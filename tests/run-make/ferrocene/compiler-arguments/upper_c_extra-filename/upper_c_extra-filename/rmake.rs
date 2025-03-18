use std::path::Path;

use run_make_support::{rfs, rustc, target};

fn main() {
    rustc().target(target()).input("main.rs").extra_filename("foo").run();

    assert!(Path::new("mainfoo").is_file())
}

// ferrocene-annotations: um_rustc_C_extra_filename
