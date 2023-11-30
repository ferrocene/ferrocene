fn main() {
    match Some("foo") {
        None::<isize> => {}   //~ ERROR mismatched types
        Some(_) => {}
    }
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
