#![allow(unused_variables)]

use std::ptr;

// Test that we only report **one** error here and that is that
// `break` with an expression is illegal in this context. In
// particular, we don't report any mismatched types error, which is
// besides the point.

fn main() {
    for _ in &[1,2,3] {
        break 22 //~ ERROR `break` with value from a `for` loop
    }
}

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
//
// ferrocene-annotations: fls_onfyolkcbeh3
// For Loops
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
