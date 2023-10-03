#![feature(never_type)]

fn main() {
    // The `if false` expressions are simply to
    // make sure we don't avoid checking everything
    // simply because a few expressions are unreachable.

    if false {
        let _: ! = { //~ ERROR mismatched types
            'a: while break 'a {};
        };
    }

    if false {
        let _: ! = {
            while false { //~ ERROR mismatched types
                break
            }
        };
    }

    if false {
        let _: ! = {
            while false { //~ ERROR mismatched types
                return
            }
        };
    }
}

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
//
// ferrocene-annotations: fls_5jjm1kt43axd
// While Loops
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
