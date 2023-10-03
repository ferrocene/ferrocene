// Ensure that capturing closures are never coerced to fns
// Especially interesting as non-capturing closures can be.

fn main() {
    let b = 0u8;
    let baz: fn() -> u8 = (|| { b }) as fn() -> u8;
    //~^ ERROR non-primitive cast
}

// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
//
// ferrocene-annotations: fls_jmjn8jkbzujm
// Capturing
