#[cfg(feature = "serialize-serde")]
extern crate syntex;
#[cfg(feature = "serialize-serde")]
extern crate serde_codegen;

use std::env;
use std::fs::File;
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
    }

    expand_serde(out_dir);
    println!("cargo:rerun-if-changed=src/capture.rs");
}

#[cfg(not(feature = "serialize-serde"))]
fn expand_serde(_out_dir: &Path) {}

#[cfg(feature = "serialize-serde")]
fn expand_serde(out_dir: &Path) {
    let dst = out_dir.join("capture.rs");

    let mut input = File::open(&dst).unwrap();
    let mut tmp = File::create(out_dir.join("tmp.rs")).unwrap();
    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();
    tmp.write_all(s.replace("//~ HACK2 ", "").as_bytes()).unwrap();

    let mut registry = syntex::Registry::new();
    serde_codegen::register(&mut registry);
    registry.expand("", &out_dir.join("tmp.rs"), &dst).unwrap();
}
