//@ run-pass

struct Foo;

trait Trait {
    fn bar(&self); //~ WARN method `bar` is never used
}

// Inherent impls should be preferred over trait ones.
impl Foo {
    fn bar(&self) {}
}

impl dyn Trait {
    fn baz(_: &Foo) {}
}

impl Trait for Foo {
    fn bar(&self) { panic!("wrong method called!") }
}

fn main() {
    Foo.bar();
    Foo::bar(&Foo);
    <Foo>::bar(&Foo);

    // Should work even if Trait::baz doesn't exist.
    // N.B: `<Trait>::bar` would be ambiguous.
    <dyn Trait>::baz(&Foo);
}

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
