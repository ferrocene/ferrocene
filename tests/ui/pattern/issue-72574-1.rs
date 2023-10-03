fn main() {
    let x = (1, 2, 3);
    match x {
        (_a, _x @ ..) => {}
        _ => {}
    }
}
//~^^^^ ERROR `_x @` is not allowed in a tuple
//~| ERROR: `..` patterns are not allowed here
//~| ERROR: mismatched types

// ferrocene-annotations: fls_vnai6ag4qrdb
// Identifier Pattern Matching
//
// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
