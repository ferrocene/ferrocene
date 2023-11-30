// Tests for "default" bounds inferred for traits with no bounds list.

trait Foo {}

fn a(_x: Box<dyn Foo + Send>) {
}

fn b(_x: &'static (dyn Foo + 'static)) {
}

fn c(x: Box<dyn Foo + Sync>) {
    a(x); //~ ERROR mismatched types
}

fn d(x: &'static (dyn Foo + Sync)) {
    b(x);
}

fn main() {}

// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
//
// ferrocene-annotations: fls_ikfvbeewame7
// Subtyping and Variance
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
