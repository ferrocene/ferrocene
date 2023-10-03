use std::marker::PhantomData;

struct AssertSync<T: Sync>(PhantomData<T>);

pub struct Foo {
    bar: *const Bar,
    phantom: PhantomData<Bar>,
}

pub struct Bar {
    foo: *const Foo,
    phantom: PhantomData<Foo>,
}

fn main() {
    let _: AssertSync<Foo> = unimplemented!();
    //~^ ERROR E0277
    //~| ERROR E0277
}

// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_eiw4by8z75di
// Send and Sync
