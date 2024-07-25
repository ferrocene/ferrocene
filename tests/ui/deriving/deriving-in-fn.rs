//@ run-pass

#![allow(dead_code)]

pub fn main() {
    #[derive(Debug)]
    struct Foo {
        foo: isize,
    }

    let f = Foo { foo: 10 };
    let _ = format!("{:?}", f);
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
