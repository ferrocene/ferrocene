//@ run-pass

struct J { j: isize }

pub fn main() {
    let i: Box<_> = Box::new(J {
        j: 100
    });
    assert_eq!(i.j, 100);
}

// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
