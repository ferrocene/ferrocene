//@ check-pass
//
// Check that we don't ignore private fields in usefulness checking
#![deny(unreachable_patterns)]

mod inner {
    #[derive(PartialEq, Eq)]
    pub struct PrivateField {
        pub x: bool,
        y: bool,
    }

    pub const FOO: PrivateField = PrivateField { x: true, y: true };
    pub const BAR: PrivateField = PrivateField { x: true, y: false };
}
use inner::*;

fn main() {
    match FOO {
        FOO => {}
        BAR => {}
        _ => {}
    }

    match FOO {
        FOO => {}
        PrivateField { x: true, .. } => {}
        _ => {}
    }
}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
//
// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
//
// ferrocene-annotations: fls_7wpgnp4kjq82
// Rest Patterns
//
// ferrocene-annotations: fls_asj8rgccvkoe
// Struct Pattern Matching
