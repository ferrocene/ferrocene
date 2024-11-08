//@ check-fail
//@ compile-flags: --error-format human --color always
//@ error-pattern: cannot find value `x` in this scope
//@ ignore-windows: Target uses different escape codes.

fn main() {
    let x = x;
}

// ferrocene-annotations: um_rustc_color
// ferrocene-annotations: um_rustc_error_format
