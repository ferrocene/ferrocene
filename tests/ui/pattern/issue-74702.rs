fn main() {
    let (foo @ ..,) = (0, 0);
    //~^ ERROR: `foo @` is not allowed in a tuple
    //~| ERROR: `..` patterns are not allowed here
    //~| ERROR: mismatched types
    dbg!(foo);
}

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
