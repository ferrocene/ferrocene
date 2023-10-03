enum Enum {
    A, B, C, D, E, F
}
use Enum::*;

struct S(Enum, ());
struct Sd { x: Enum, y: () }

fn main() {
    match (A, ()) { //~ ERROR non-exhaustive
        (A, _) => {}
    }

    match (A, A) { //~ ERROR non-exhaustive
        (_, A) => {}
    }

    match ((A, ()), ()) { //~ ERROR non-exhaustive
        ((A, ()), _) => {}
    }

    match ((A, ()), A) { //~ ERROR non-exhaustive
        ((A, ()), _) => {}
    }

    match ((A, ()), ()) { //~ ERROR non-exhaustive
        ((A, _), _) => {}
    }


    match S(A, ()) { //~ ERROR non-exhaustive
        S(A, _) => {}
    }

    match (Sd { x: A, y: () }) { //~ ERROR non-exhaustive
        Sd { x: A, y: _ } => {}
    }

    match Some(A) { //~ ERROR non-exhaustive
        Some(A) => (),
        None => ()
    }
}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
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
// ferrocene-annotations: fls_qfsfnql1t7m
// Underscore Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Underscore Pattern Matching
