// Issue #6155

//@ dont-require-annotations: NOTE

fn first((value, _): (isize, f64)) -> isize { value }

fn main() {
    let y = first ((1,2.0,3));
    //~^ ERROR mismatched types
    //~| NOTE expected tuple `(isize, f64)`
    //~| NOTE found tuple `(isize, f64, {integer})`
    //~| NOTE expected a tuple with 2 elements, found one with 3 elements

    let y = first ((1,));
    //~^ ERROR mismatched types
    //~| NOTE expected tuple `(isize, f64)`
    //~| NOTE found tuple `(isize,)`
    //~| NOTE expected a tuple with 2 elements, found one with 1 element
}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
// ferrocene-annotations: fls_k64tfywtn0g8
// Tuple Expressions
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
// ferrocene-annotations: fls_4ckl3n2ko3i4
// Tuple Types
