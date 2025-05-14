#[derive(PartialEq, Eq)]
enum X { A, B, C, }

fn main() {
    let x = X::A;
    let y = Some(X::A);

    match (x, y) {
        //~^ ERROR non-exhaustive patterns: `(X::A, Some(X::A))`, `(X::A, Some(X::B))`, `(X::B, Some(X::B))` and 2
        //~| NOTE more not covered
        //~| NOTE the matched value is of type `(X, Option<X>)`
        (_, None) => false,
        (v, Some(w)) if v == w => true,
        (X::B, Some(X::C)) => false,
        (X::B, Some(X::A)) => false,
        (X::A, Some(X::C)) | (X::C, Some(X::A)) => false,
    };
}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
//
// ferrocene-annotations: fls_uloyjbaso8pz
// Path Patterns
//
// ferrocene-annotations: fls_d44aflefat88
// Path Pattern Matching
