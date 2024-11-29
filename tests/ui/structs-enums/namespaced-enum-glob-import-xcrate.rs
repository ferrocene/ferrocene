//@ run-pass
//@ aux-build:namespaced_enums.rs


extern crate namespaced_enums;

fn _f(f: namespaced_enums::Foo) {
    use namespaced_enums::Foo::*;

    match f {
        A | B(_) | C { .. } => {}
    }
}

mod m {
    pub use namespaced_enums::Foo::*;
}

fn _f2(f: namespaced_enums::Foo) {
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
