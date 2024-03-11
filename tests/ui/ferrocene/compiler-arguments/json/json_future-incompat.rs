//@ check-pass
//@ compile-flags: --error-format=json --json=future-incompat

//@ From https://github.com/rust-lang/rust/issues/79813

macro_rules! foo {
    () => {
        true;
    };
}

fn main() {
    let val = match true {
        true => false,
        _ => foo!(),
    };
}

// ferrocene-annotations: um_rustc_json
// ferrocene-annotations: um_rustc_error_format
