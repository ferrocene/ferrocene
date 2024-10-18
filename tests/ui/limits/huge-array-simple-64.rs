//@ build-fail
//@ ignore-32bit

#![allow(arithmetic_overflow)]

fn main() {
    let _fat: [u8; (1<<61)+(1<<31)] = //~ ERROR too big for the target architecture
        [0; (1u64<<61) as usize +(1u64<<31) as usize];
}

// ferrocene-annotations: fls_xinykul167l
// Array Expressions
//
// ferrocene-annotations: fls_uj0kpjwyld60
// Array Type
//
// ferrocene-annotations: fls_1qhsun1vyarz
// Type Cast Expressions
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
