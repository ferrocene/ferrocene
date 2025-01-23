//@ revisions: current next
//@[next] compile-flags: -Znext-solver
//@ ignore-compare-mode-next-solver (explicit revisions)
// Test that a where clause attached to a method allows us to add
// additional constraints to a parameter out of scope.

struct Foo<T> {
    value: T,
}

struct Bar; // does not implement Eq

impl<T> Foo<T> {
    fn equals(&self, u: &Foo<T>) -> bool
    where
        T: Eq,
    {
        self.value == u.value
    }
}

fn main() {
    let x = Foo { value: Bar };
    x.equals(&x);
    //~^ ERROR `Bar: Eq` is not satisfied
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
