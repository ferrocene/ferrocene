// ferrocene-annotations: um_rustc_C_linker_flavor
//@ only-linux
//@ only-x86_64

use run_make_support::{rustc, target};

fn main() {
    assert_linker("cc", &["-C", "linker-flavor=gcc"]);
    assert_linker("ld", &["-C", "linker-flavor=ld"]);
}

#[track_caller]
fn assert_linker(linker: &str, extra_args: &[&str]) {
    assert!(
        rustc()
            .input("main.rs")
            .print("link-args")
            .args(extra_args)
            .run_unchecked()
            .stdout_utf8()
            .contains(&format!("\"{linker}\""))
    );
}
