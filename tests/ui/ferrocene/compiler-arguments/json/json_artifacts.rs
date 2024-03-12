//@ build-pass
//@ compile-flags: --crate-type=rlib --error-format=json --json=artifacts

fn factorial(value: i32) -> i32 {
    match value {
        i32::MIN ..= 1 => { 1 }
        _ => { value * factorial(value - 1) }
    }
}

// ferrocene-annotations: um_rustc_json
// ferrocene-annotations: um_rustc_error_format
