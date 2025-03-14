// ferrocene-annotations: um_rustc_C_link_args

use run_make_support::{rustc, target};

fn main() {
    let result = rustc()
        .input("main.rs")
        .target(target())
        .print("link-args")
        .args(["-C", "link-args=-lfoo -lbar"])
        .run_unchecked()
        .stdout_utf8();
    assert!(result.contains("-lfoo") && result.contains("-lbar"));
}
