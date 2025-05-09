//@ check-fail
//@ compile-flags: --error-format=json --json=diagnostic-rendered-ansi

fn main() {
    let x = x; //~ ERROR cannot find value `x` in this scope
}

// ferrocene-annotations: um_rustc_json
// ferrocene-annotations: um_rustc_error_format
