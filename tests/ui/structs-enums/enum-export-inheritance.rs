//@ run-pass
#![allow(dead_code)]

mod a {
    pub enum Foo {
        Bar,
        Baz,
        Boo
    }
}

pub fn main() {
    let _x = a::Foo::Bar;
}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_dq403wq5yrs
// Namespaces
