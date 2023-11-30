fn main() {
    let a: u16;
    let b: u16 = 42;
    let c: usize = 5;

    a = c + b * 5; //~ ERROR: mismatched types [E0308]
                   //~| ERROR: mismatched types [E0308]
                   //~| ERROR: cannot add `u16` to `usize` [E0277]
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
