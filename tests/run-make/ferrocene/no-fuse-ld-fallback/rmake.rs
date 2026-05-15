// only test targets that use `-fuse-ld` by default
//@ needs-linker-flavor-lld-cc

//! Check that if Ferrocene's rustc is used with a linker driver that does not support the
//! `-fuse-ld` flag then it will produce an error instead of re-invoking the linker *without*
//! the `-fuse-ld` flag, which is a fallback that upstream rustc implements
//!
//! # Test design
//!
//! This test does *not* use the `-C linker` flag as this affects whether `-fuse-ld` is passed to
//! the linker or not. The scenario tested here is with*out* the `-C linker` flag.
//!
//! Therefore, to override the default system linker, e.g. `cc`, we override the
//! $PATH environment variable so that a stub linker (see `./linker.rs`) is called instead of the
//! system linker.
//!
//! The stub linker will produce the error message that upstream rustc uses to implement the
//! `-fuse-ld` fallback when it sees the `-fuse-ld` argument. Otherwise, the stub linker
//! panics which indicates that the Ferrocene logic is wrong or the setup test is wrong.

use std::env;
use std::path::Path;

use run_make_support::{Diff, bare_rustc, target};

fn main() {
    let out_dir = Path::new("out");
    let mut output = bare_rustc()
        .target(target())
        .crate_type("bin")
        .input("main.rs")
        .print("link-args")
        .out_dir(&out_dir)
        .run();

    let stdout = output.stdout_utf8();
    assert!(
        stdout.contains("-fuse-ld="),
        "this target does not use `-fuse-ld`; add to ignore list"
    );

    // `--print link-args` output has the form:
    // "ENVVAR1=value1" "ENVVAR2=value2" "the-linker" "link-arg1" "link-arg2" (..)
    //                                    ^^^^^^^^^^
    // we'll extract the linker argument
    let linker_driver_arg = stdout
        .split("\" ")
        .skip_while(|fragment| fragment.contains('='))
        .next()
        .expect("could not extract linker argument")
        .strip_prefix('"')
        .expect("linker argument was not inside double quotes");

    assert!(
        matches!(linker_driver_arg, "cc"),
        "unexpected linker driver; logic may need to be revised"
    );

    build_linker_driver_override_for_host(out_dir, linker_driver_arg);

    let original_path = env::var("PATH").expect("could not read $PATH variable");
    let new_path = format!("{}:{original_path}", out_dir.display());

    let mut output = bare_rustc()
        .target(target())
        .crate_type("bin")
        .input("main.rs")
        .override_path(&new_path)
        .out_dir(&out_dir)
        .run_fail();

    Diff::new().expected_file("rustc.stderr").actual_text("rustc", output.stderr_utf8()).run();
}

fn build_linker_driver_override_for_host(out_dir: &Path, linker_driver_arg: &str) {
    bare_rustc().crate_type("bin").input("linker.rs").output(out_dir.join(linker_driver_arg)).run();
}
