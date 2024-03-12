//@ [full] run-pass
//@ revisions: full min
#![cfg_attr(full, feature(adt_const_params))]
#![cfg_attr(full, allow(incomplete_features))]

struct Foo {
    i: i32,
}

trait Get<'a, const N: &'static str> {
    //[min]~^ ERROR `&'static str` is forbidden as the type of a const generic parameter
    type Target: 'a;

    fn get(&'a self) -> &'a Self::Target;
}

impl Foo {
    fn ask<'a, const N: &'static str>(&'a self) -> &'a <Self as Get<N>>::Target
    //[min]~^ ERROR `&'static str` is forbidden as the type of a const generic parameter
    where
        Self: Get<'a, N>,
    {
        self.get()
    }
}

impl<'a> Get<'a, "int"> for Foo {
    type Target = i32;

    fn get(&'a self) -> &'a Self::Target {
        &self.i
    }
}

fn main() {
    let foo = Foo { i: 123 };
    assert_eq!(foo.ask::<"int">(), &123);
}

// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation conformance

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
