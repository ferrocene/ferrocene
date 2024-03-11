//@ run-pass

struct Foo {
    new: isize,
}

pub fn main() {
    let foo = Foo{ new: 3 };
    assert_eq!(foo.new, 3);
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
