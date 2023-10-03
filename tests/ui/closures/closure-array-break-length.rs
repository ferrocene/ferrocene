fn main() {
    |_: [_; continue]| {}; //~ ERROR: `continue` outside of a loop

    while |_: [_; continue]| {} {} //~ ERROR: `continue` outside of a loop

    while |_: [_; break]| {} {} //~ ERROR: `break` outside of a loop
}

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
//
// ferrocene-annotations: fls_sjwrlwvpulp
// Continue Expressions
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
