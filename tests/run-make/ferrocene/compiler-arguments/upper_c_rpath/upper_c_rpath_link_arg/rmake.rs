// ferrocene-annotations: um_rustc_C_rpath

use run_make_support::{rustc, target};

fn main() {
    assert!(!passes_rpath(&[]));
    assert!(passes_rpath(&["-Crpath=yes"]));
    assert!(!passes_rpath(&["-Crpath=no"]));
    assert!(passes_rpath(&["-Crpath"]));
}

fn passes_rpath(extra_args: &[&str]) -> bool {
    rustc()
        .target(target())
        .input("main.rs")
        .print("link-args")
        .crate_type("bin")
        .args(extra_args)
        .run()
        .stdout_utf8()
        .contains("-rpath")
}
