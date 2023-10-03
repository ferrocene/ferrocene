#![feature(type_alias_impl_trait)]

fn main() {}

trait T {}
impl T for i32 {}

fn should_ret_unit() -> impl T {
    //~^ ERROR `(): T` is not satisfied
    panic!()
}

type Foo = impl T;

fn a() -> Foo {
    //~^ ERROR `(): T` is not satisfied
    panic!()
}

fn b() -> Foo {
    42
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
