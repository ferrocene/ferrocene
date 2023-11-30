// revisions: mir thir
// [thir]compile-flags: -Z thir-unsafeck

fn f(p: *const u8) -> u8 {
    return *p; //~ ERROR dereference of raw pointer is unsafe
}

fn main() {
}

// ferrocene-annotations: fls_ppd1xwve3tr7
// Raw Pointer Types
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
