//@ aux-build:namespaced_enums.rs
extern crate namespaced_enums;

mod m {
    pub use namespaced_enums::Foo::*;
}

pub fn main() {
    use namespaced_enums::Foo::*;

    foo(); //~ ERROR cannot find function `foo` in this scope
    m::foo(); //~ ERROR cannot find function `foo` in module `m`
    bar(); //~ ERROR cannot find function `bar` in this scope
    m::bar(); //~ ERROR cannot find function `bar` in module `m`
}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
