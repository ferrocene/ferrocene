// Emits two asm files, one with no-vectorize-loops enabled and one without.
// Optimization is on for both, just no-vectorize-loops is different.
// The resulting asm must be different.

use std::path::Path;

use run_make_support::{bare_rustc, rfs as fs};

fn main() {
    let input = Path::new("example.rs");
    let output = input.with_extension("s");

    let with = Path::new("out-dir-with");
    fs::create_dir(with);
    bare_rustc()
        .input(input)
        .crate_type("lib")
        .opt()
        .emit("asm")
        .arg("-C")
        .arg("no-vectorize-loops")
        .out_dir(with)
        .run();

    let without = Path::new("out-dir-without");
    fs::create_dir(without);
    bare_rustc().input(input).crate_type("lib").opt().emit("asm").out_dir(without).run();

    assert_ne!(fs::read_to_string(with.join(&output)), fs::read_to_string(without.join(&output)))
}

// ferrocene-annotations: um_rustc_C_no_vectorize_loops
