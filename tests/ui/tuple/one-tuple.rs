//@ run-pass
// Why one-tuples? Because macros.


pub fn main() {
    match ('c',) {
        (x,) => {
            assert_eq!(x, 'c');
        }
    }
    // test the 1-tuple type too
    let x: (char,) = ('d',);
    let (y,) = x;
    assert_eq!(y, 'd');
}

// ferrocene-annotations: fls_k64tfywtn0g8
// Tuple Expressions
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
// ferrocene-annotations: fls_4ckl3n2ko3i4
// Tuple Types
