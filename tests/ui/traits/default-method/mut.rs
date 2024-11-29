//@ check-pass
#![allow(unused_assignments)]

#![allow(unused_variables)]

trait Foo {
    fn foo(&self, mut v: isize) { v = 1; }
}

pub fn main() {}

// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
