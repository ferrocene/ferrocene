static mut a: isize = 3;

fn main() {
    unsafe {
        a = true; //~ ERROR: mismatched types
    }
}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
