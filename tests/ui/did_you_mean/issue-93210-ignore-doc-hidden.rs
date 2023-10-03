#[derive(Default)]
pub struct A {
    #[doc(hidden)]
    pub hello: i32,
    pub bye: i32,
}

#[derive(Default)]
pub struct B {
    pub hello: i32,
    pub bye: i32,
}

fn main() {
    A::default().hey;
    //~^ ERROR no field `hey` on type `A`
    //~| NOTE unknown field
    //~| NOTE available fields are: `bye`

    B::default().hey;
    //~^ ERROR no field `hey` on type `B`
    //~| NOTE unknown field
    //~| NOTE available fields are: `hello`, `bye`
}

// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
