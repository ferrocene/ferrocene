<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
pub enum EFoo {
    A,
}

pub trait Foo {
    const X: EFoo;
}

struct Abc;

impl Foo for Abc {
    const X: EFoo = EFoo::A;
}

struct Def;
impl Foo for Def {
    const X: EFoo = EFoo::A;
}

pub fn test<A: Foo, B: Foo>(arg: EFoo, A::X: EFoo) {
    //~^ ERROR associated consts cannot be referenced in patterns
    let A::X = arg;
    //~^ ERROR associated consts cannot be referenced in patterns
}

fn main() {}

// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
