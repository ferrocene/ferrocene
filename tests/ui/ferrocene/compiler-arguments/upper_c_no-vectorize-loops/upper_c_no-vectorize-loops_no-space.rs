// As other codegen options, no-vectorize-loops should also work without a space after -C.

//@ check-pass
//@ edition: 2021
//@ compile-flags: -Cno-vectorize-loops

fn main() {}

// ferrocene-annotations: um_rustc_C_no_vectorize_loops
