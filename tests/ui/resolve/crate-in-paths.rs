//@ edition:2018

mod bar {
    pub(crate) struct Foo;
}

fn main() {
    Foo;
    //~^ ERROR cannot find value `Foo` in this scope [E0425]
}

// ferrocene-annotations: fls_maw4u1o8q37u
// Crates
