fn main() {
    match 1i32 {
        1i32 => 1,
        2u32 => 1, //~ ERROR mismatched types
        _ => 2,
    };
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
