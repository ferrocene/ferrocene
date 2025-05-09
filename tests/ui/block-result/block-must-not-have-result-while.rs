//@ dont-require-annotations: NOTE

fn main() {
    while true { //~ WARN denote infinite loops with
        true //~  ERROR mismatched types
             //~| NOTE expected `()`, found `bool`
    }
}

// ferrocene-annotations: fls_hndm19t57wby
// Block Expressions
//
// ferrocene-annotations: fls_5jjm1kt43axd
// While Loops
