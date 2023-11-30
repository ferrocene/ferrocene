// aux-build:lint-group-plugin-test.rs
// check-pass
// compile-flags: -D unused -A unused-variables

fn main() {
    let x = 1;
}

// ferrocene-annotations: um_rustc_D
