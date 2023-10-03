fn mismatched_types_with_reference(x: &u32) -> &u32 {
    if false {
        return x;
    }
    return "test";
    //~^ERROR mismatched types
}

fn main() {}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
