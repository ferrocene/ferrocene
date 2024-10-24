//@ ignore-64bit
//@ build-fail

#![allow(arithmetic_overflow)]

fn main() {
    let _fat: [u8; (1<<31)+(1<<15)] = //~ ERROR too big for the target architecture
        [0; (1u32<<31) as usize +(1u32<<15) as usize];
}

// ferrocene-annotations: fls_xinykul167l
// Array Expressions
//
// ferrocene-annotations: fls_uj0kpjwyld60
// Array Type
//
// ferrocene-annotations: fls_1qhsun1vyarz
// Type Cast Expressions
