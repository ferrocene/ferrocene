fn main() {
    let mut x = &0;
    let mut y = &0;
    (x, y) = &(1, 2); //~ ERROR mismatched types
}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
