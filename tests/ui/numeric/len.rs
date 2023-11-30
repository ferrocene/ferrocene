fn main() {
    let array = [1, 2, 3];
    test(array.len()); //~ ERROR mismatched types
}

fn test(length: u32) {
    println!("{}", length);
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
