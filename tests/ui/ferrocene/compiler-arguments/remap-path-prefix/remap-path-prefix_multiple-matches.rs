//@ Test what happens when both the first and the second FROM match
//
//@ check-fail
//@ compile-flags: --remap-path-prefix={{src-base}}=foo --remap-path-prefix={{src-base}}=bar

fn main() {
    //@ We cannot actually put an ERROR marker here because
    //@ the file name in the error message is not what the
    //@ test framework expects (since the filename gets remapped).
    //@ We still test the expected error in the stderr file.
    this_error_is_here_to_generate_stderr_file
}

// ferrocene-annotations: um_rustc_remap_path_prefix
