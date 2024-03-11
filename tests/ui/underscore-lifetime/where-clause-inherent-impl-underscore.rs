//@ revisions: rust2015 rust2018
//@[rust2018] edition:2018

trait WithType<T> {}
trait WithRegion<'a> { }

struct Foo<T> {
    t: T
}

impl<T> Foo<T>
where
    T: WithRegion<'_>
//[rust2015,rust2018]~^ ERROR `'_` cannot be used here
{ }

fn main() {}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
