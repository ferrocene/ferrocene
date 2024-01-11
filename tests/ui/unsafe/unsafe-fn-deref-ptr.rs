fn f(p: *const u8) -> u8 {
    let _ = *p; //~ ERROR dereference of raw pointer is unsafe
    let _: u8 = *p; //~ ERROR dereference of raw pointer is unsafe
    _ = *p; //~ ERROR dereference of raw pointer is unsafe
    return *p; //~ ERROR dereference of raw pointer is unsafe
}

fn main() {
}

// ferrocene-annotations: fls_ppd1xwve3tr7
// Raw Pointer Types
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
