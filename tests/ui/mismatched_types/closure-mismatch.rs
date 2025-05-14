//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[next] compile-flags: -Znext-solver

trait Foo {}

impl<T: Fn(&())> Foo for T {}

fn baz<T: Foo>(_: T) {}

fn main() {
    baz(|_| ());
    //[current]~^ ERROR implementation of `FnOnce` is not general enough
    //[current]~| ERROR implementation of `Fn` is not general enough
    //[next]~^^^ ERROR Foo` is not satisfied
    baz(|x| ());
    //[current]~^ ERROR implementation of `FnOnce` is not general enough
    //[current]~| ERROR implementation of `Fn` is not general enough
    //[next]~^^^ ERROR Foo` is not satisfied
}

// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_xd2oxlebhs14
// Closure Types
