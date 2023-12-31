// Test that we can quantify lifetimes outside a constraint (i.e., including
// the self type) in a where clause. Specifically, test that we cannot nest
// quantification in constraints (to be clear, there is no reason this should not
// we're testing we don't crash or do something stupid).

trait Bar<'a> {
    fn bar(&self);
}

impl<'a, 'b> Bar<'b> for &'a u32 {
    fn bar(&self) {}
}

fn foo<T>(x: &T)
    where for<'a> &'a T: for<'b> Bar<'b>
    //~^ error: nested quantification of lifetimes
{}

fn main() {}

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
