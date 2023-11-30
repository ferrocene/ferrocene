enum A {
    B { x: Option<isize> },
    C
}

fn main() {
    let x = A::B { x: Some(3) };
    match x {   //~ ERROR non-exhaustive patterns
        A::C => {}
        A::B { x: None } => {}
    }
}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_uloyjbaso8pz
// Path Patterns
//
// ferrocene-annotations: fls_d44aflefat88
// Path Pattern Matching
