use std::path::Path;

use run_make_support::{bare_rustc, target};

fn main() {
    let out_dir = Path::new("out");
    bare_rustc()
        .edition("2021")
        .target(target())
        .crate_type("bin")
        .input("main.rs")
        .out_dir(&out_dir)
        .run();

    assert!(out_dir.is_dir());
    assert!(out_dir.join("main").is_file());
}

// ferrocene-annotations: um_rustc_out_dir
