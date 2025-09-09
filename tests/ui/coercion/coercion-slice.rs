// Tests that we forbid coercion from `[T; n]` to `&[T]`

fn main() {
    let _: &[i32] = [0];
    //~^ ERROR mismatched types
    //~| NOTE expected `&[i32]`, found `[{integer}; 1]`
    //~| NOTE expected due to this
}

// ferrocene-annotations: fls_uj0kpjwyld60
// Array Types
//
// ferrocene-annotations: fls_vpbikb73dw4k
// Slice Types
