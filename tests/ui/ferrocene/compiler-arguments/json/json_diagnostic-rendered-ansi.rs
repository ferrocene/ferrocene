//@ check-fail
//@ compile-flags: --error-format=json --json=diagnostic-rendered-ansi
//@ error-pattern: cannot find value `x` in this scope
//@ ignore-windows: Target uses different escape codes.

fn main() {
    let x = x;
}

// ferrocene-annotations: um_rustc_json
// ferrocene-annotations: um_rustc_error_format
