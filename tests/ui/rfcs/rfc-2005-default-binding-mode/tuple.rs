//@ run-pass
pub fn main() {
    let foo = (Some(1), (), (), vec![2, 3]);

    match &foo {
        (Some(n), .., v) => {
            assert_eq!((*v).len(), 2);
            assert_eq!(*n, 1);
        }
        (None, (), (), ..) => panic!(),
    }
}

// ferrocene-annotations: fls_7wpgnp4kjq82
// Rest Patterns
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
