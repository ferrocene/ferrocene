fn main() {
    [(); & { loop { continue } } ]; //~ ERROR mismatched types

    [(); loop { break }]; //~ ERROR mismatched types

    [(); {while true {break}; 0}];
    //~^ WARN denote infinite loops with

    [(); { for _ in 0usize.. {}; 0}];
    //~^ ERROR `for` is not allowed in a `const`
    //~| ERROR cannot convert
    //~| ERROR cannot call
}

// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
// ferrocene-annotations: fls_66m4rnbssgig
// Constant Expressions
