//@ run-pass
#![allow(dead_code)]
#![allow(non_snake_case)]


pub mod Foo {
    pub trait Trait {
        fn foo(&self);
    }
}

mod Bar {
    impl<'a> dyn (::Foo::Trait) + 'a {
        fn bar(&self) { self.foo() }
    }
}

fn main() {}

// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
