//@ check-fail
//@ compile-flags: -C no-vectorize-loops=false
//~? incorrect value

fn main() {}

// ferrocene-annotations: um_rustc_C_no_vectorize_loops
