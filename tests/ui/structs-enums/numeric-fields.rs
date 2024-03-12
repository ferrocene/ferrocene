//@ run-pass
struct S(u8, u16);

fn main() {
    let s = S{1: 10, 0: 11};
    match s {
        S{0: a, 1: b, ..} => {
            assert_eq!(a, 11);
            assert_eq!(b, 10);
        }
    }
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
