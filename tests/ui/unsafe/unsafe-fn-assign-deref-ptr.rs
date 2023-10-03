// revisions: mir thir
// [thir]compile-flags: -Z thir-unsafeck

fn f(p: *mut u8) {
    *p = 0; //~ ERROR dereference of raw pointer is unsafe
    return;
}

fn main() {
}

// ferrocene-annotations: fls_ppd1xwve3tr7
// Raw Pointer Types
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
