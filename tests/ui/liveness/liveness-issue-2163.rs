use std::vec::Vec;

fn main() {
    let a: Vec<isize> = Vec::new();
    a.iter().all(|_| -> bool {
        //~^ ERROR mismatched types
    });
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
