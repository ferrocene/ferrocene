//@ check-pass
//@ compile-flags: --emit=asm -o {{build-base}}//output_filename
//~? WARN multiple output types requested

fn main() {}

// ferrocene-annotations: um_rustc_emit
