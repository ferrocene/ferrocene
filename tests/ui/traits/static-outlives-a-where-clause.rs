//@ run-pass

trait Foo<'a> {
    fn xyz(self);
}
impl<'a, T> Foo<'a> for T where 'static: 'a {
    fn xyz(self) {}
}

trait Bar {
    fn uvw(self);
}
impl<T> Bar for T where for<'a> T: Foo<'a> {
    fn uvw(self) { self.xyz(); }
}

fn foo<T>(t: T) where T: Bar {
    t.uvw();
}

fn main() {
    foo(0);
}

// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_ikfvbeewame7
// Subtyping and Variance
