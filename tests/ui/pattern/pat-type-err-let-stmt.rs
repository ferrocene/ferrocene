// Test the `.span_label` to the type / scrutinee
// when there's a type error in checking a pattern.

fn main() {
    // We want to point at the `Option<u8>`.
    let Ok(0): Option<u8> = 42u8;
    //~^ ERROR mismatched types
    //~| ERROR mismatched types

    // We want to point at the `Option<u8>`.
    let Ok(0): Option<u8>;
    //~^ ERROR mismatched types

    // We want to point at the scrutinee.
    let Ok(0) = 42u8; //~ ERROR mismatched types
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
