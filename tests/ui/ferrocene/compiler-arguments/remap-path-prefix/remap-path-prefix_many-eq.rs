//@ check-fail
//@ compile-flags: --remap-path-prefix======

fn main() {
    this_error_is_here_to_generate_stderr_file //~ ERROR
}

// ferrocene-annotations: um_rustc_remap_path_prefix
