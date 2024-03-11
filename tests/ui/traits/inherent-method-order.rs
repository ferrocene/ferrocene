//@ run-pass

struct Foo;

impl Foo {
    #[allow(dead_code)]
    fn foo(self) {
        panic!("wrong method!")
    }
}

trait Trait {
    fn foo(self);
}

impl<'a,'b,'c> Trait for &'a &'b &'c Foo {
    fn foo(self) {
        // ok
    }
}

fn main() {
    let x = &(&(&Foo));
    x.foo();
}

// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
