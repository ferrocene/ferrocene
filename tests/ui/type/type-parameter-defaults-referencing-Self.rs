// Test a default that references `Self` which is then used in an object type.
// Issue #18956.

trait Foo<T=Self> {
    fn method(&self);
}

fn foo(x: &dyn Foo) { }
//~^ ERROR the type parameter `T` must be explicitly specified

fn main() { }

// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
// ferrocene-annotations: fls_1qhsun1vyarz
// Type Cast Expressions
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
