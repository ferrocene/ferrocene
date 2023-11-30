trait Foo {
    type Output<T>;

    fn baz();
}

enum Bar<T> {
    Simple {},
    Generic(T),
}

impl Foo for u8 {
    type Output<T> = Bar<T>;
    fn baz() {
        Self::Output::Simple {}; //~ ERROR type annotations needed
    }
}

fn main() {}

// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
//
// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
