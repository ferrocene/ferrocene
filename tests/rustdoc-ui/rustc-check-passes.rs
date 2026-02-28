//@ check-pass

#![feature(rustdoc_internals)]
#![feature(rustdoc_internals)] //~ WARN duplicate

pub fn foo() {}
