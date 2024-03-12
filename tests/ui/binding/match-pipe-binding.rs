//@ run-pass

fn test1() {
    // from issue 6338
    match ((1, "a".to_string()), (2, "b".to_string())) {
        ((1, a), (2, b)) | ((2, b), (1, a)) => {
                assert_eq!(a, "a".to_string());
                assert_eq!(b, "b".to_string());
            },
            _ => panic!(),
    }
}

fn test2() {
    match (1, 2, 3) {
        (1, a, b) | (2, b, a) => {
            assert_eq!(a, 2);
            assert_eq!(b, 3);
        },
        _ => panic!(),
    }
}

fn test3() {
    match (1, 2, 3) {
        (1, ref a, ref b) | (2, ref b, ref a) => {
            assert_eq!(*a, 2);
            assert_eq!(*b, 3);
        },
        _ => panic!(),
    }
}

fn test4() {
    match (1, 2, 3) {
        (1, a, b) | (2, b, a) if a == 2 => {
            assert_eq!(a, 2);
            assert_eq!(b, 3);
        },
        _ => panic!(),
    }
}

fn test5() {
    match (1, 2, 3) {
        (1, ref a, ref b) | (2, ref b, ref a) if *a == 2 => {
            assert_eq!(*a, 2);
            assert_eq!(*b, 3);
        },
        _ => panic!(),
    }
}

pub fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
}

// ferrocene-annotations: fls_5loglxds6zik
// Parenthesized pattern matching
//
// ferrocene-annotations: fls_1xit18et4ohh
// Parenthesized patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple pattern matching
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple patterns
