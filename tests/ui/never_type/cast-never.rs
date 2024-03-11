// Test that we can explicitly cast ! to another type

//@ check-pass

#![feature(never_type)]

fn main() {
    let x: ! = panic!();
    let y: u32 = x as u32;
}

// ferrocene-annotations: fls_1qhsun1vyarz
// Type Cast Expressions
