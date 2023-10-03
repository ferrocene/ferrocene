union U {
    a: usize,
    b: usize,
}

const C: U = U { a: 10 };

fn main() {
    match C {
        C => {} //~ ERROR cannot use unions in constant patterns
        _ => {}
    }
}

// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
