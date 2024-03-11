//@ check-pass
// Regression test for #78507.
fn foo() -> Option<fn() -> Option<bool>> {
    Some(|| Some(true))
}
fn main() {}

// ferrocene-annotations: fls_xztr1kebz8bo
// Function Pointer Types
