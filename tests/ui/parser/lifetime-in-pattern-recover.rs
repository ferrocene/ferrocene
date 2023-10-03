fn main() {
    let &'a x = &0; //~ ERROR unexpected lifetime `'a` in pattern
    let &'a mut y = &mut 0; //~ ERROR unexpected lifetime `'a` in pattern

    let _recovery_witness: () = 0; //~ ERROR mismatched types
}

// ferrocene-annotations: fls_yivm43r5wnp1
// Let Statements
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
