//@ check-pass

// This used to cause a stack overflow during exhaustiveness checking in the compiler.

fn main() {
    const LARGE_SIZE: usize = 1024 * 1024;
    let [..] = [0u8; LARGE_SIZE];
    match [0u8; LARGE_SIZE] {
        [..] => {}
    }
}

// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_qte70mgzpras
// Slice Patterns
//
// ferrocene-annotations: fls_57ic33pwdvp3
// Slice Pattern Matching
//
// ferrocene-annotations: fls_xinykul167l
// Array Expressions
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
