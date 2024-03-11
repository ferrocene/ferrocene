//@ run-pass

#![allow(irrefutable_let_patterns)]

fn main() {
    if let _ = 5 {}

    while let _ = 5 {
        break;
    }
}

// ferrocene-annotations: fls_uh76pw6ykd57
// Refutability
