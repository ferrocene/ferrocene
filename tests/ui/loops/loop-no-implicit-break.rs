fn main() {
    let a: i8 = loop {
        1 //~ ERROR mismatched types
    };

    let b: i8 = loop {
        break 1;
    };
}

fn foo() -> i8 {
    let a: i8 = loop {
        1 //~ ERROR mismatched types
    };

    let b: i8 = loop {
        break 1;
    };

    loop {
        1 //~ ERROR mismatched types
    }

    loop {
        return 1;
    }

    loop {
        1 //~ ERROR mismatched types
    }
}

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
//
// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
