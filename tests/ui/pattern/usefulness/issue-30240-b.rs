#![deny(unreachable_patterns)]

fn main() {
    match "world" {
        "hello" => {}
        _ => {},
    }

    match "world" {
        ref _x if false => {}
        "hello" => {}
        "hello" => {} //~ ERROR unreachable pattern
        _ => {},
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
// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
//
// ferrocene-annotations: fls_vnai6ag4qrdb
// Identifier Pattern Matching
