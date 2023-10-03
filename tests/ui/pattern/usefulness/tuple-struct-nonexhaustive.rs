struct Foo(isize, isize);

fn main() {
    let x = Foo(1, 2);
    match x {   //~ ERROR non-exhaustive
        Foo(1, b) => println!("{}", b),
        Foo(2, b) => println!("{}", b)
    }
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
//
// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
//
// ferrocene-annotations: fls_eexupzdsu7f
// Tuple Struct Pattern Matching
