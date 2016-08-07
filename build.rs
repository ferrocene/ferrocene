#[cfg(feature = "serialize-serde")]
extern crate serde_codegen;

use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

// See src/capture.rs for what in the world this build script is doing.

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);

    if cfg!(feature = "serialize-rustc") {
        let mut s = String::new();
        File::open("src/capture.rs").unwrap()
             .read_to_string(&mut s).unwrap();
        let s = s.replace("//~ HACK1 ", "");
        File::create(out_dir.join("capture.rs")).unwrap()
             .write_all(s.as_bytes()).unwrap();
    } else {
        fs::copy("src/capture.rs", out_dir.join("capture.rs")).unwrap();
    }

    expand_serde(out_dir);
    println!("cargo:rerun-if-changed=src/capture.rs");
}

#[cfg(not(feature = "serialize-serde"))]
fn expand_serde(_out_dir: &Path) {}

#[cfg(feature = "serialize-serde")]
fn expand_serde(out_dir: &Path) {
    use std::thread;

    let dst = out_dir.join("capture.rs");

    let mut input = File::open(&dst).unwrap();
    let mut tmp = File::create(out_dir.join("tmp.rs")).unwrap();
    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();
    tmp.write_all(s.replace("//~ HACK2 ", "").as_bytes()).unwrap();

    // This has been seen to overflow the stack on travis, so just use a
    // dedicated big-stack thread.
    let out_dir = out_dir.to_path_buf();
    thread::Builder::new().stack_size(16 * 1024 * 1024).spawn(move || {
        serde_codegen::expand(&out_dir.join("tmp.rs"), &dst).unwrap();
    }).unwrap().join().unwrap();
}
