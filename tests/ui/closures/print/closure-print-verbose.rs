//@ compile-flags: -Zverbose-internals

// Same as closure-coerce-fn-1.rs

// Ensure that capturing closures are never coerced to fns
// Especially interesting as non-capturing closures can be.

fn main() {
    let mut a = 0u8;
    let foo: fn(u8) -> u8 = |v: u8| { a += v; a };
    //~^ ERROR mismatched types
}

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
// ferrocene-annotations: fls_jmjn8jkbzujm
// Capturing
