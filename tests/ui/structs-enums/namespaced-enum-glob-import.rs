//@ run-pass
#![allow(dead_code)]

mod m2 {
    pub enum Foo {
        A,
        B(isize),
        C { a: isize },
    }

    impl Foo {
        pub fn foo() {}
    }
}

mod m {
    pub use m2::Foo::*;
}

fn _f(f: m2::Foo) {
    use m2::Foo::*;

    match f {
        A | B(_) | C { .. } => {}
    }
}

fn _f2(f: m2::Foo) {
    match f {
        m::A | m::B(_) | m::C { .. } => {}
    }
}

pub fn main() {}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_dq403wq5yrs
// Namespaces
