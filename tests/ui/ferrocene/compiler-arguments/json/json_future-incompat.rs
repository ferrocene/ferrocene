// check-fail
// compile-flags: --error-format=json --json=future-incompat

#[repr(packed)]
struct S {
    a: u8,
    b: u16,
}

fn f(s: &S) {
    &s.b;
}

fn main() {}

// ferrocene-annotations: um_rustc_json
// ferrocene-annotations: um_rustc_error_format
