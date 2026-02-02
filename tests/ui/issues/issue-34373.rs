#![allow(warnings)]

trait Trait<T> {
    fn foo(_: T) {}
}

pub struct Foo<T = Box<dyn Trait<DefaultFoo>>>;  //~ ERROR cycle detected
//~^ ERROR `T` is never used
//~| ERROR cycle detected
type DefaultFoo = Foo;

fn main() {
}
