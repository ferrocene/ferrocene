//@ dont-require-annotations: NOTE

enum Foo {
    Bar { bar: Bar, id: usize }
}

enum Bar {
    A, B, C, D, E, F
}

fn test(f: Foo) {
    match f {
        //~^ ERROR non-exhaustive patterns
        //~| NOTE patterns
        Foo::Bar { bar: Bar::A, .. } => (),
        Foo::Bar { bar: Bar::B, .. } => (),
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
//
// ferrocene-annotations: fls_7wpgnp4kjq82
// Rest Patterns
//
// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
//
// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
//
// ferrocene-annotations: fls_asj8rgccvkoe
// Struct Pattern Matching
