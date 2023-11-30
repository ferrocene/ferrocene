// check-fail
// compile-flags: --error-format human

fn main() {
    let x = x; //~ ERROR cannot find value `x` in this scope
}

// ferrocene-annotations: um_rustc_error_format
