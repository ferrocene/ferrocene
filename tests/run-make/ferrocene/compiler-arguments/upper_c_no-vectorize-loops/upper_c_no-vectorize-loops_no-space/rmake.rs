// As other codegen options, no-vectorize-loops should also work without a space after -C.

use run_make_support::bare_rustc;

fn main() {
    bare_rustc()
        .input("main.rs")
        .edition("2021")
        .crate_type("bin")
        .arg("-Cno-vectorize-loops")
        .run();
}

// ferrocene-annotations: um_rustc_C_no_vectorize_loops
