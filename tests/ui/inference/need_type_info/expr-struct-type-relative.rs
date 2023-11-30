// regression test for #98598

trait Foo {
    type Output;

    fn baz() -> Self::Output;
}

fn needs_infer<T>() {}

struct Bar {}

impl Foo for u8 {
    type Output = Bar;
    fn baz() -> Self::Output {
        needs_infer(); //~ ERROR type annotations needed
        Self::Output {}
    }
}

fn main() {}

// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
//
// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
