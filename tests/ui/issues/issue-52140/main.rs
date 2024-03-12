//@ run-pass
//@ aux-build:some_crate.rs
//@ compile-flags:--extern some_crate
//@ edition:2018

mod foo {
    pub use some_crate;
}

fn main() {
    ::some_crate::hello();
    foo::some_crate::hello();
}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
