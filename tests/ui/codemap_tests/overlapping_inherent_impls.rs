// Test that you cannot define items with the same name in overlapping inherent
// impl blocks.

#![allow(unused)]

struct Foo;

impl Foo {
    fn id() {} //~ ERROR duplicate definitions
}

impl Foo {
    fn id() {}
}

struct Bar<T>(T);

impl<T> Bar<T> {
    fn bar(&self) {} //~ ERROR duplicate definitions
}

impl Bar<u32> {
    fn bar(&self) {}
}

struct Baz<T>(T);

impl<T: Copy> Baz<T> {
    fn baz(&self) {} //~ ERROR duplicate definitions
}

impl<T> Baz<Vec<T>> {
    fn baz(&self) {}
}

fn main() {}

// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
//
// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
