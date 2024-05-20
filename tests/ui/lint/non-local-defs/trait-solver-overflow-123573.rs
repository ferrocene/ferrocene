//@ check-pass
//@ edition:2021

#![warn(non_local_definitions)]

// https://github.com/rust-lang/rust/issues/123573#issue-2229428739

pub trait Test {}

impl<'a, T: 'a> Test for &[T] where &'a T: Test {}

fn main() {
    struct Local {}
    impl Test for &Local {}
    //~^ WARN non-local `impl` definition
}
