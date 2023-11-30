enum P {
    C(PC),
}

enum PC {
    Q,
    QA,
}

fn test(proto: P) {
    match proto { //~ ERROR non-exhaustive patterns
        P::C(PC::Q) => (),
    }
}

fn main() {}

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
