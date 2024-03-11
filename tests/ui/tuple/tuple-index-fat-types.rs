//@ run-pass

struct Foo<'a>(&'a [isize]);

fn main() {
    let x: &[isize] = &[1, 2, 3];
    let y = (x,);
    assert_eq!(y.0, x);

    let x: &[isize] = &[1, 2, 3];
    let y = Foo(x);
    assert_eq!(y.0, x);
}

// ferrocene-annotations: fls_k64tfywtn0g8
// Tuple Expressions
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
