//@ run-pass

fn main() {
    let x = 1;

    #[cfg(false)]
    if false {
        x = 2;
    } else if true {
        x = 3;
    } else {
        x = 4;
    }
    assert_eq!(x, 1);
}

// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
// ferrocene-annotations: fls_u1afezy1ye99
// Conditional Compilation
// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
