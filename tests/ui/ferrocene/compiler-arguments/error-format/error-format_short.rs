//@ check-fail
//@ compile-flags: --error-format short
//~? RAW cannot find value `x` in this scope

fn main() {
    let x = x;
}

// ferrocene-annotations: um_rustc_error_format
