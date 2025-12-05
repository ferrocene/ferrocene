//@ edition:2015..2021
#![feature(type_alias_impl_trait)]

fn main() {}

trait T {
    type Assoc;
}

type Foo = impl T;

#[define_opaque(Foo)]
fn a() -> Foo {
    //~^ ERROR the trait bound `(): T` is not satisfied
    // This is not a defining use, it doesn't actually constrain the opaque type.
    panic!()
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
