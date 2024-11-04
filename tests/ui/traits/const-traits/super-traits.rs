//@ check-pass
//@ compile-flags: -Znext-solver
#![allow(incomplete_features)]
#![feature(const_trait_impl, effects)]

#[const_trait]
trait Foo {
    fn a(&self);
}

#[const_trait]
trait Bar: ~const Foo {}

struct S;
impl const Foo for S {
    fn a(&self) {}
}

impl const Bar for S {}

const fn foo<T: ~const Bar>(t: &T) {
    t.a();
}

const _: () = foo(&S);

fn main() {}
