//@ check-pass
#![feature(custom_inner_attributes)]
#![deny(ferrocene::unvalidated)]
#![ferrocene::prevalidated]

fn foo() {}

fn main() {
    foo();
}
