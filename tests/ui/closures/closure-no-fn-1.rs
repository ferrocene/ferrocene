// Ensure that capturing closures are never coerced to fns
// Especially interesting as non-capturing closures can be.

fn main() {
    let mut a = 0u8;
    let foo: fn(u8) -> u8 = |v: u8| { a += v; a };
    //~^ ERROR mismatched types
}

// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
//
// ferrocene-annotations: fls_jmjn8jkbzujm
// Capturing
