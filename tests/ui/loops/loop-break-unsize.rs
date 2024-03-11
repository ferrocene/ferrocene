// Regression test for #62312
//@ check-pass

fn main() {
    let _ = loop {
        break Box::new(()) as Box<dyn Send>;
    };
}

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
//
// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
//
// ferrocene-annotations: fls_1qhsun1vyarz
// Type Cast Expressions
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
