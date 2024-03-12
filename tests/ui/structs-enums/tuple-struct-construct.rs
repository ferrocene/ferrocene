//@ run-pass
#[allow(dead_code)]
#[derive(Debug)]
struct Foo(isize, isize);

pub fn main() {
    let x = Foo(1, 2);
    println!("{:?}", x);
}

// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
