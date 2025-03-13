// ferrocene-annotations: um_rustc_C_relocation_model

use run_make_support::{Rustc, rustc, target};

fn main() {
    compiler().arg("-Crelocation-model=default").run();
    compiler().arg("-Crelocation-model=pic").run();
    compiler().arg("-Crelocation-model=pie").run();
    compiler().arg("-Crelocation-model=static").run();

    compiler().arg("-Crelocation-model").run_fail();
    compiler().arg("-Crelocation-model=").run_fail();
    compiler().arg("-Crelocation-model=invalid-relocation-model").run_fail();
}

fn compiler() -> Rustc {
    let mut compiler = rustc();
    compiler.input("main.rs").target(target()).crate_type("bin");
    compiler
}
