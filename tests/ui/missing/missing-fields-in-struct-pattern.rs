struct S(usize, usize, usize, usize);

fn main() {
    if let S { a, b, c, d } = S(1, 2, 3, 4) {
    //~^ ERROR tuple variant `S` written as struct variant
        println!("hi");
    }
}

// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple pattern matching
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple patterns
//
// ferrocene-annotations: fls_jm6l7b90h6wa
// Pattern matching
//
// ferrocene-annotations: fls_p0t1ch115tra
// If Let Expressions
//
// ferrocene-annotations: fls_asj8rgccvkoe
// Struct Pattern Matching
//
// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
