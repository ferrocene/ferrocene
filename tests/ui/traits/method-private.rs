//@ edition:2015
mod inner {
    pub trait Bar {
        fn method(&self);
    }

    pub struct Foo;

    impl Foo {
        fn method(&self) {}
    }

    impl Bar for Foo {
        fn method(&self) {}
    }
}

fn main() {
    let foo = inner::Foo;
    foo.method(); //~ ERROR is private
}

// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
