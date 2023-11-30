fn main() {
    match "world" { //~ ERROR non-exhaustive patterns: `&_`
        "hello" => {}
    }

    match "world" { //~ ERROR non-exhaustive patterns: `&_`
        ref _x if false => {}
        "hello" => {}
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
