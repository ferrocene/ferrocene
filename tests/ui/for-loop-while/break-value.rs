//@ run-pass
#![allow(unreachable_code)]

fn int_id(x: isize) -> isize { return x; }

pub fn main() { loop { int_id(break); } }

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
//
// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
