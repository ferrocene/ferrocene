// ferrocene-annotations: um_rustc_L
//@ ignore-remote Temporary ignore, while we fix a bug in the test infrastructure.

use run_make_support::rfs::create_dir_all;
use run_make_support::{bare_rustc, cc, llvm_ar, run, target};

// bare_rustc() is used thorough this test to prevent run_make_utils from configuring its library
// search path (that's what we are testing!).

fn main() {
    bare_rustc()
        .input("operations.rs")
        .crate_type("lib")
        .out_dir("operations_dir")
        .target(target())
        .run();

    bare_rustc()
        .input("factorial.rs")
        .crate_type("lib")
        .out_dir("factorial_dir")
        .args(["-L", "crate=operations_dir"])
        .target(target())
        .run();

    create_dir_all("native_dir");
    cc().input("operations.c").output("native_dir/liboperations.o").arg("-c").run();
    llvm_ar()
        .obj_to_ar()
        .output_input("native_dir/liboperations.a", "native_dir/liboperations.o")
        .run();

    compile_and_run("L", "main_operations.rs", &["-L", "operations_dir"]);
    compile_and_run("Lall", "main_operations.rs", &["-L", "all=operations_dir"]);
    compile_and_run("Lcrate", "main_operations.rs", &["-L", "crate=operations_dir"]);

    compile_and_run(
        "Ldependency",
        "main_factorial.rs",
        &["-L", "crate=factorial_dir", "-L", "dependency=operations_dir"],
    );

    compile_and_run("Lnative", "main_native.rs", &["-L", "native=native_dir", "-l", "operations"]);
}

fn compile_and_run(name: &str, input: &str, extra_args: &[&str]) {
    bare_rustc()
        .input(input)
        .output(name)
        .crate_type("bin")
        .args(extra_args)
        .target(target())
        .run();
    run(name);
}
