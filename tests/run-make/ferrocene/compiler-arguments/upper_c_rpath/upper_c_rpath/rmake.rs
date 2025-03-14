//@ ignore-apple On Mac, signatures cause the binaries to differ
//@ needs-dynamic-linking
// ferrocene-annotations: um_rustc_C_rpath

use run_make_support::rfs::read;
use run_make_support::{run_in_tmpdir, rustc, target};

fn main() {
    let default = compile_and_read(&[]);
    let yes = compile_and_read(&["-Crpath=yes"]);
    let no = compile_and_read(&["-Crpath=no"]);
    let empty = compile_and_read(&["-Crpath"]);

    assert!(default == no);
    assert!(empty == yes);
    assert!(yes != no);
}

fn compile_and_read(extra_flags: &[&str]) -> Vec<u8> {
    let mut content = None;
    run_in_tmpdir(|| {
        rustc().target(target()).input("main.rs").crate_type("bin").args(extra_flags).run();
        content = Some(read("main"));
    });
    content.unwrap()
}
