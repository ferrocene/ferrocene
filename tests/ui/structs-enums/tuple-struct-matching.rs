//@ run-pass
struct Foo(isize, isize);

pub fn main() {
    let x = Foo(1, 2);
    match x {
        Foo(a, b) => {
            assert_eq!(a, 1);
            assert_eq!(b, 2);
            println!("{} {}", a, b);
        }
    }
}

// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
//
// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
//
// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
//
// ferrocene-annotations: fls_eexupzdsu7f
// Tuple Struct Pattern Matching
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
