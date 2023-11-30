fn main() {
    let tup = (true, true);
    println!("foo {:}", match tup { //~ ERROR non-exhaustive patterns: `(true, false)` not covered
        (false, false) => "foo",
        (false, true) => "bar",
        (true, true) => "baz"
    });
}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
