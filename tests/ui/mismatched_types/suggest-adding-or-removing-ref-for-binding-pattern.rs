//@ run-rustfix
#![allow(dead_code, unused_variables)]

fn main() {
    enum Blah {
        A(isize, isize, usize),
        B(isize, usize),
    }

    match Blah::A(1, 1, 2) {
        Blah::A(_, x, ref y) | Blah::B(x, y) => {}
        //~^ ERROR mismatched types
        //~| ERROR variable `y` is bound inconsistently across alternatives separated by `|`
    }

    match Blah::A(1, 1, 2) {
        Blah::A(_, x, y) | Blah::B(x, ref y) => {}
        //~^ ERROR mismatched types
        //~| ERROR variable `y` is bound inconsistently across alternatives separated by `|`
    }
}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_jm6l7b90h6wa
// Pattern Matching
//
// ferrocene-annotations: fls_xgqh0ju6bmbn
// Patterns
