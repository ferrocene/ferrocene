trait Foo<T> {
    fn dummy(&self, t: T) { }
}

trait Bar<A> {
    fn method<B>(&self) where A: Foo<B>;
}

struct S;
struct X;

// Remove this impl causing the below resolution to fail // impl Foo<S> for X {}

impl Bar<X> for isize {
    fn method<U>(&self) where X: Foo<U> {
    }
}

fn main() {
    1.method::<X>();
    //~^ ERROR the trait bound `X: Foo<X>` is not satisfied
}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
