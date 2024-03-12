//@ run-pass

pub fn main() {
    let i: Box<_> = Box::new(vec![100]);
    assert_eq!((*i)[0], 100);
}

// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
//
// ferrocene-annotations: fls_g0uyl7qw4c7w
// Parenthesized Expressions
//
// ferrocene-annotations: fls_sxcr4aa098i6
// Indexing Expressions
