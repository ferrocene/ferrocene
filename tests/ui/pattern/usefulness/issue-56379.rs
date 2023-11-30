enum Foo {
    A(bool),
    B(bool),
    C(bool),
}

fn main() {
    match Foo::A(true) {
        //~^ ERROR non-exhaustive patterns: `Foo::A(false)`, `Foo::B(false)` and `Foo::C(false)` not covered
        Foo::A(true) => {}
        Foo::B(true) => {}
        Foo::C(true) => {}
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
