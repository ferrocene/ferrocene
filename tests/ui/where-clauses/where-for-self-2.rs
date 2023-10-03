// Test that we can quantify lifetimes outside a constraint (i.e., including
// the self type) in a where clause. Specifically, test that implementing for a
// specific lifetime is not enough to satisfy the `for<'a> ...` constraint, which
// should require *all* lifetimes.

static X: &'static u32 = &42;

trait Bar {
    fn bar(&self);
}

impl Bar for &'static u32 {
    fn bar(&self) {}
}

fn foo<T>(x: &T)
where
    for<'a> &'a T: Bar,
{
}

fn main() {
    foo(&X); //~ ERROR implementation of `Bar` is not general enough
}

// ferrocene-annotations: fls_xdvdl2ssnhlo
// Statics
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
