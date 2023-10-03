#![allow(warnings)]

fn closure_expecting_bound<F>(_: F)
where
    F: FnOnce(&u32),
{
}

fn expect_bound_supply_named<'x>() {
    let mut f: Option<&u32> = None;

    // Here we give a type annotation that `x` should be free. We get
    // an error because of that.
    closure_expecting_bound(|x: &'x u32| {
        //~^ ERROR lifetime may not live long enough
        //~| ERROR lifetime may not live long enough

        // Borrowck doesn't get a chance to run, but if it did it should error
        // here.
        f = Some(x);
    });
}

fn main() {}

// ferrocene-annotations: fls_airvr79xkcag
// Function Item Type
//
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_xd2oxlebhs14
// Closure Type
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_l9ebxrlxyawd
// Lifetime Elision
//
// ferrocene-annotations: fls_i7g2n7hfg3ch
// Generic Conformance
