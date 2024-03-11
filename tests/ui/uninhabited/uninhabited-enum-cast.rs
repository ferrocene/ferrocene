//@ check-pass

enum E {}

fn f(e: E) {
    println!("{}", (e as isize).to_string());
}

fn main() {}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_1qhsun1vyarz
// Type Cast Expressions
//
// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
