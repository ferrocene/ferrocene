//@ run-pass
//@ compile-flags: --remap-path-prefix={{src-base}}/ferrocene/compiler-arguments/remap-path-prefix=remapped

fn main() {
    let this_file = std::path::PathBuf::from(file!());
    let expected = std::path::PathBuf::from("remapped/remap-path-prefix_macro-expansion.rs");
    assert_eq!(expected, this_file);
}

// ferrocene-annotations: um_rustc_remap_path_prefix
