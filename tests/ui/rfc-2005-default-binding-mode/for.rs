struct Foo {}

pub fn main() {
    let mut tups = vec![(Foo {}, Foo {})];
    // The below desugars to &(ref n, mut m).
    for (n, mut m) in &tups {
        //~^ ERROR cannot move out of a shared reference
    }
}

// ferrocene-annotations: fls_onfyolkcbeh3
// For Loops
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
