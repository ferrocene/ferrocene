//@ check-pass
// Tests that type parameter bounds on an implementation need not match the
// trait exactly, as long as the implementation doesn't demand *more* bounds
// than the trait.


trait A {
    fn foo<T: Eq + Ord>(&self);
}

impl A for isize {
    fn foo<T: Ord>(&self) {} // Ord implies Eq, so this is ok.
}

fn main() {}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
