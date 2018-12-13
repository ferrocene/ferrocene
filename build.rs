extern crate autocfg;

fn main() {
    let ac = autocfg::new();

    // ffi types moved from `std` to `core` in Rust 1.30, so we need to adjust imports based on
    // this.
    //
    // <https://github.com/rust-lang/rust/pull/53910>
    ac.emit_rustc_version(1, 30);

    println!("cargo:rerun-if-changed=build.rs");
}
