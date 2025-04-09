//@ build-fail
//@ ignore-wasi wasi does different things with the `main` symbol
//
// See #67946.

#![allow(warnings)]
fn main() { //~ ERROR entry symbol `main` declared multiple times
    extern "Rust" {
     fn main();
    }
    unsafe { main(); }
}

// ferrocene-annotations: fls_osd6c4utyjb3
// FFI
//
// ferrocene-annotations: fls_pgp7ezcc9lh8
// Foreign Function Interface Attributes
