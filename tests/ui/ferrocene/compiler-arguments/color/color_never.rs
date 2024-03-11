//@ check-fail
//@ compile-flags: --error-format human --color never
//@ error-pattern: cannot find value `x` in this scope

fn main() {
    let x = x;
}

// ferrocene-annotations: um_rustc_color
// ferrocene-annotations: um_rustc_error_format
