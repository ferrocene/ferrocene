//@ check-pass
// From https://github.com/rust-lang/rust/issues/78549

fn main() {
    match "foo" {
        "foo" => {},
        &_ => {},
    }

    match "foo" {
        &_ => {},
        "foo" => {},
    }

    match ("foo", 0, "bar") {
        (&_, 0, &_) => {},
        ("foo", _, "bar") => {},
        (&_, _, &_) => {},
    }

    match (&"foo", "bar") {
        (&"foo", &_) => {},
        (&&_, &_) => {},
    }
}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_boyhlu5srp6u
// String Literals
//
// ferrocene-annotations: fls_hucd52suu6it
// Simple String Literals
//
// ferrocene-annotations: fls_d2sc9hl3v0mk
// Reference Patterns
//
// ferrocene-annotations: fls_org6hqv397fp
// Reference Pattern Matching
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
//
// ferrocene-annotations: fls_k64tfywtn0g8
// Tuple Expressions
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
