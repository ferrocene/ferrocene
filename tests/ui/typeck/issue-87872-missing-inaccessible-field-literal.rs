pub mod foo {
    pub struct Foo {
        pub you_can_use_this_field: bool,
        you_cant_use_this_field: bool,
    }
}

fn main() {
    foo::Foo {};
    //~^ ERROR cannot construct `Foo` with struct literal syntax due to private fields
}

// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
