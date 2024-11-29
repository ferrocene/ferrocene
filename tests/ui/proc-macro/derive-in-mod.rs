//@ check-pass
//@ proc-macro: test-macros.rs

extern crate test_macros;

mod inner {
    use test_macros::Empty;

    #[derive(Empty)]
    struct S;
}

fn main() {}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
