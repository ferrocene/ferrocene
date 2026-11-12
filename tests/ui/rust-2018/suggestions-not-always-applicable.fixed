//@ proc-macro: suggestions-not-always-applicable.rs
//@ edition:2015
//@ run-rustfix
//@ rustfix-only-machine-applicable
//@ check-pass

#![warn(rust_2018_compatibility)]

extern crate suggestions_not_always_applicable as foo;

pub struct Foo;

mod test {
    use crate::foo::foo;

    #[foo]
    fn main() {}
}

fn main() {
    test::foo();
}

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
//
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
