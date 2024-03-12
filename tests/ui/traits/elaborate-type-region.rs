//@ run-pass
#![allow(dead_code)]

// Test that we elaborate `Type: 'region` constraints and infer various important things.

trait Master<'a, T: ?Sized> {
    fn foo() where T: 'a;
}

// [U]: 'a => U: 'a
impl<'a, U> Master<'a, [U]> for () {
    fn foo() where U: 'a { }
}

// &'b U: 'a => 'b: 'a, U: 'a
impl<'a, 'b, U> Master<'a, &'b U> for () {
    fn foo() where 'b: 'a, U: 'a { }
}

// &'b [U]: 'a => 'b: 'a, U: 'a
impl<'a, 'b, U> Master<'a, &'b [U]> for () {
    fn foo() where 'b: 'a, U: 'a { }
}

// Foo<'b>: 'a => 'b: 'a
struct Foo<'a> { x: &'a () }
impl<'a, 'b> Master<'a, Foo<'b>> for () {
    fn foo() where 'b: 'a { }
}

// Bar<'b, T>: 'a => 'b: 'a, T: 'a
struct Bar<'a, T: 'a> { x: &'a T }
impl<'a, 'b, T> Master<'a, Bar<'b, T>> for () {
    fn foo() where 'b: 'a, T: 'a { }
}

// fn(T): 'a => T: 'a
impl<'a, T> Master<'a, fn(T)> for () {
    fn foo() where T: 'a { }
}

// fn() -> T: 'a => T: 'a
impl<'a, T> Master<'a, fn() -> T> for () {
    fn foo() where T: 'a { }
}

fn main() {
    println!("Hello, world!");
}

// ferrocene-annotations: fls_ikfvbeewame7
// Subtyping and Variance
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
