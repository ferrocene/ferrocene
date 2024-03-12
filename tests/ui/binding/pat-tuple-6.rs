//@ run-pass
fn tuple() {
    let x = (1, 2, 3, 4, 5);
    match x {
        (a, .., b, c) => {
            assert_eq!(a, 1);
            assert_eq!(b, 4);
            assert_eq!(c, 5);
        }
    }
    match x {
        (a, b, c, .., d) => {
            assert_eq!(a, 1);
            assert_eq!(b, 2);
            assert_eq!(c, 3);
            assert_eq!(d, 5);
        }
    }
}

fn tuple_struct() {
    struct S(u8, u8, u8, u8, u8);

    let x = S(1, 2, 3, 4, 5);
    match x {
        S(a, .., b, c) => {
            assert_eq!(a, 1);
            assert_eq!(b, 4);
            assert_eq!(c, 5);
        }
    }
    match x {
        S(a, b, c, .., d) => {
            assert_eq!(a, 1);
            assert_eq!(b, 2);
            assert_eq!(c, 3);
            assert_eq!(d, 5);
        }
    }
}

fn main() {
    tuple();
    tuple_struct();
}

// ferrocene-annotations: fls_7wpgnp4kjq82
// Rest patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple pattern matching
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple patterns
//
// ferrocene-annotations: fls_eexupzdsu7f
// Tuple struct pattern matching
//
// ferrocene-annotations: fls_vlrto778v49m
// Tuple struct patterns
