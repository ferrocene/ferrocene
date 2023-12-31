// Test that an object type `Box<Foo>` is not considered to implement the
// trait `Foo`. Issue #5087.

trait Foo {}
fn take_foo<F:Foo>(f: F) {}
fn take_object(f: Box<dyn Foo>) { take_foo(f); }
//~^ ERROR `Box<dyn Foo>: Foo` is not satisfied
fn main() {}

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
