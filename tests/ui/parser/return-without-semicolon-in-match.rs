// Tests that `return` without a semicolon parses correctly in a match arm.
// See <https://github.com/rust-lang/rust/issues/521>
//
//@ check-pass

fn _f() {
    #[rustfmt::skip]
    let _x = match true {
        true => { 10 },
        false => { return },
    };
}

fn main() {}

// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
