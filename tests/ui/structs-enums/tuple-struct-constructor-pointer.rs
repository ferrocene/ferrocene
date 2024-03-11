//@ run-pass
#[derive(PartialEq, Debug)]
struct Foo(isize);
#[derive(PartialEq, Debug)]
struct Bar(isize, isize);

pub fn main() {
    let f: fn(isize) -> Foo = Foo;
    let g: fn(isize, isize) -> Bar = Bar;
    assert_eq!(f(42), Foo(42));
    assert_eq!(g(4, 7), Bar(4, 7));
}

// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
