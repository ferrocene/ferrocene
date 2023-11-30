trait Foo {
    type Output;

    fn baz() -> Self::Output;
}

fn needs_infer<T>() {}

enum Bar {
    Variant {}
}

impl Foo for u8 {
    type Output = Bar;
    fn baz() -> Self::Output {
        needs_infer(); //~ ERROR type annotations needed
        Self::Output::Variant {}
    }
}

fn main() {}

// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
//
// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
