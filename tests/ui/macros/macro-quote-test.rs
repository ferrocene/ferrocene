// Test that a macro can emit delimiters with nothing inside - `()`, `{}`

//@ run-pass
//@ proc-macro: hello_macro.rs

extern crate hello_macro;

fn main() {
    hello_macro::hello!();
}

// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
//
// ferrocene-annotations: fls_wn1i6hzg2ff7
// Procedural Macros
//
// ferrocene-annotations: fls_2d6bqnpy6tvs
// Function-like Macros
