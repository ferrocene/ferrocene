//@ check-pass

union U {
    a: usize,
    b: usize,
}

const C: U = U { a: 10 };

fn main() {
    let a: [u8; unsafe { C.a }];
    let b: [u8; unsafe { C.b }];
}

// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
//
// ferrocene-annotations: fls_8wnyln2nmg4y
// Unsafe Blocks
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_uj0kpjwyld60
// Array Types
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
