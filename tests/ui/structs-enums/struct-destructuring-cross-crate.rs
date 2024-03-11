//@ run-pass
//@ aux-build:struct_destructuring_cross_crate.rs


extern crate struct_destructuring_cross_crate;

pub fn main() {
    let x = struct_destructuring_cross_crate::S { x: 1, y: 2 };
    let struct_destructuring_cross_crate::S { x: a, y: b } = x;
    assert_eq!(a, 1);
    assert_eq!(b, 2);
}

// ferrocene-annotations: fls_maw4u1o8q37u
// Crates
//
// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
//
// ferrocene-annotations: fls_yivm43r5wnp1
// Let Statements
//
// ferrocene-annotations: fls_asj8rgccvkoe
// Struct Pattern Matching
