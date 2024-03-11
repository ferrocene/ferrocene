//@ run-pass
#![allow(dead_code)]
// test for issue #30244

#[derive(Copy, Clone)]
struct Array {
    arr: [[u8; 256]; 4]
}

pub fn main() {}

// ferrocene-annotations: fls_uj0kpjwyld60
// Array Type
//
// ferrocene-annotations: fls_xinykul167l
// Array Expressions
