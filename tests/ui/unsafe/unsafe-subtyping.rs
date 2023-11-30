// Check that safe fns are not a subtype of unsafe fns.

fn foo(x: Option<fn(i32)>) -> Option<unsafe fn(i32)> {
    x //~ ERROR mismatched types
}

fn bar(x: fn(i32)) -> unsafe fn(i32) {
    x // OK, coercion!
}

fn main() { }

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_ikfvbeewame7
// Subtyping and Variance
