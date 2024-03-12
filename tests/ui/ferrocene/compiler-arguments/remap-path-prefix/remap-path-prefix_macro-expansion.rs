//@ run-pass
//@ compile-flags: --remap-path-prefix={{src-base}}/ferrocene/compiler-arguments/remap-path-prefix=remapped

fn main() {
    let this_file = file!();
    assert_eq!("remapped/remap-path-prefix_macro-expansion.rs", this_file);
}

// ferrocene-annotations: um_rustc_remap_path_prefix
