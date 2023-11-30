trait Foo {
    pub const Foo: u32;
    //~^ ERROR unnecessary visibility qualifier
}

fn main() {}

// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
