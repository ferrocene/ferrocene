//@ run-pass

struct X { x: isize }

pub fn main() {
    let x: Box<_> = Box::new(X {x: 1});
    let bar = x;
    assert_eq!(bar.x, 1);
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
