<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
//@ edition:2015..2021
#![feature(type_alias_impl_trait)]

trait T {
    type Assoc: Cake;
}

trait Cake: std::fmt::Display {
    fn cake() -> Self;
}

type Foo = impl T;

fn foo() -> impl T {
    //~^ ERROR `(): T` is not satisfied
    panic!()
}

#[define_opaque(Foo)]
fn a() -> Foo {
    foo()
}

fn main() {
    println!("{}", <Foo as T>::Assoc::cake());
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
