trait Foo {
    pub type Foo;
    //~^ ERROR visibility qualifiers are not permitted here
}

fn main() {}

// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
