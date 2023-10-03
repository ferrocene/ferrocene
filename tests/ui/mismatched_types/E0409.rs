fn main() {
    let x = (0, 2);

    match x {
        (0, ref y) | (y, 0) => {} //~ ERROR E0409
                                  //~| ERROR E0308
        _ => ()
    }
}

// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_jm6l7b90h6wa
// Pattern Matching
//
// ferrocene-annotations: fls_vnai6ag4qrdb
// Identifier Pattern Matching
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
