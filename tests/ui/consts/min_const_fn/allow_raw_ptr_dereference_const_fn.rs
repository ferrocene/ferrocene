//@ check-pass

use std::ptr;

const fn test_fn(x: *const i32) {
    let x2 = unsafe { ptr::addr_of!(*x) };
}

fn main() {}

// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
