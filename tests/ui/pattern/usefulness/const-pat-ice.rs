//@ check-pass

const FOO: &&&u32 = &&&42;

fn main() {
    match unimplemented!() {
        &&&42 => {},
        FOO => {},
        _ => {},
    }
}

// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_d2sc9hl3v0mk
// Reference Patterns
//
// ferrocene-annotations: fls_org6hqv397fp
// Reference Pattern Matching
