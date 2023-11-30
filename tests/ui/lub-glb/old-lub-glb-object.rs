// Test that we give a note when the old LUB/GLB algorithm would have
// succeeded but the new code (which is stricter) gives an error.

trait Foo<T, U> {}

fn foo(x: &dyn for<'a, 'b> Foo<&'a u8, &'b u8>, y: &dyn for<'a> Foo<&'a u8, &'a u8>) {
    let z = match 22 {
        0 => x,
        _ => y,
        //~^ ERROR mismatched types
        //~| ERROR mismatched types
    };
}

fn bar(x: &dyn for<'a, 'b> Foo<&'a u8, &'b u8>, y: &dyn for<'a> Foo<&'a u8, &'a u8>) {
    // Accepted with explicit case:
    let z = match 22 {
        0 => x as &dyn for<'a> Foo<&'a u8, &'a u8>,
        _ => y,
    };
}

fn main() {}

// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetime
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
