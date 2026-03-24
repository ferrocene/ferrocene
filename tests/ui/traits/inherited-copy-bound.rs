// Test that Copy bounds inherited by trait are checked.

use std::any::Any;

trait Foo: Copy {
    fn foo(&self) {}
}

impl<T: Copy> Foo for T {}

fn take_param<T: Foo>(foo: &T) {}

fn a() {
    let x: Box<_> = Box::new(3);
    take_param(&x); //~ ERROR E0277
}

fn b() {
    let x: Box<_> = Box::new(3);
    let y = &x;
    let z = &x as &dyn Foo;
    //~^ ERROR E0038
}

<<<<<<< HEAD:tests/ui/kindck/kindck-inherited-copy-bound.rs
fn main() { }

// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
//
// ferrocene-annotations: fls_i7g2n7hfg3ch
// Generic Conformance
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_77scxuomlbgs
// Passing Conventions
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
||||||| 6bdc342ddb0:tests/ui/kindck/kindck-inherited-copy-bound.rs
fn main() { }
=======
fn main() {}
>>>>>>> pull-upstream-temp--do-not-use-for-real-code:tests/ui/traits/inherited-copy-bound.rs
