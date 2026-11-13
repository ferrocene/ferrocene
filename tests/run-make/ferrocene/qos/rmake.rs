// only test targets that use `-fuse-ld` by default
//@ only-qnx

use std::env;
use std::path::Path;

use run_make_support::{Diff, bare_rustc, cmd, target};

fn main() {
    let out_dir = Path::new("out");
    let target = target();
    let mut output = bare_rustc()
        .target(&target)
        .crate_type("bin")
        .input("main.rs")
        .print("link-args")
        .out_dir(&out_dir)
        .run();

    let is_qos = target.contains("-qos");

    let stdout = output.stdout_utf8();
    let passes_lsocket_to_linker = stdout.contains("-lsocket");
    assert!(
        (is_qos && !passes_lsocket_to_linker) || (!is_qos && passes_lsocket_to_linker),
        "this target must not pass `-lsocket` to the linker"
    );

    let readelf_output = cmd("readelf").arg("-d").arg(out_dir.join("main")).run();
    // sanity check the output format
    readelf_output.assert_stdout_contains("[libc.so");
    // actual check
    if is_qos {
        readelf_output.assert_stdout_not_contains("libsocket");
    } else {
        readelf_output.assert_stdout_contains("libsocket");
    }
}
