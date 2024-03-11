//@ run-pass
// Check that we allow a cast to `_` so long as the target type can be
// inferred elsewhere.

pub fn main() {
    let i: *const i32 = 0 as _;
    assert!(i.is_null());
}

// ferrocene-annotations: fls_s45k21yn4qur
// Inferred Type
