trait Foo {
    pub const Foo: u32;
    //~^ ERROR visibility qualifiers are not permitted here
}

fn main() {}

// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
