//@ run-pass
//@ aux-build:namespaced_enums.rs


extern crate namespaced_enums;

use namespaced_enums::Foo;

fn _foo (f: Foo) {
    match f {
        Foo::A | Foo::B(_) | Foo::C { .. } => {}
    }
}

pub fn main() {}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_dq403wq5yrs
// Namespaces
