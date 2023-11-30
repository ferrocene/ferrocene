trait Foo {
    pub type Foo;
    //~^ ERROR unnecessary visibility qualifier
}

fn main() {}

// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
