#![allow(warnings)]

fn closure_expecting_bound<F>(_: F)
where
    F: FnOnce(&u32),
{
}

fn closure_expecting_free<'a, F>(_: F)
where
    F: FnOnce(&'a u32),
{
}

fn expect_bound_supply_nothing() {
    // Because `x` is inferred to have a bound region, we cannot allow
    // it to escape into `f`:
    let mut f: Option<&u32> = None;
    closure_expecting_bound(|x| {
        f = Some(x); //~ ERROR borrowed data escapes outside of closure
    });
}

fn expect_bound_supply_bound() {
    // Because `x` is inferred to have a bound region, we cannot allow
    // it to escape into `f`, even with an explicit type annotation on
    // closure:
    let mut f: Option<&u32> = None;
    closure_expecting_bound(|x: &u32| {
        f = Some(x); //~ ERROR borrowed data escapes outside of closure
    });
}

fn expect_free_supply_nothing() {
    let mut f: Option<&u32> = None;
    closure_expecting_free(|x| f = Some(x)); // OK
}

fn expect_free_supply_bound() {
    let mut f: Option<&u32> = None;

    // Here, even though the annotation `&u32` could be seen as being
    // bound in the closure, we permit it to be defined as a free
    // region (which is inferred to something in the fn body).
    closure_expecting_free(|x: &u32| f = Some(x)); // OK
}

fn expect_free_supply_named<'x>() {
    let mut f: Option<&u32> = None;

    // Here, even though the annotation `&u32` could be seen as being
    // bound in the closure, we permit it to be defined as a free
    // region (which is inferred to something in the fn body).
    closure_expecting_free(|x: &'x u32| f = Some(x)); // OK
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
// ferrocene-annotations: fls_i7g2n7hfg3ch
// Generic Conformance
