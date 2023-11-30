// FIXME(tschottdorf): this test should pass.

#[derive(PartialEq, Eq)]
struct Foo {
    bar: i32,
}

const FOO: Foo = Foo{bar: 5};

fn main() {
    let f = Foo{bar:6};

    match &f {
        FOO => {}, //~ ERROR mismatched types
        _ => panic!(),
    }
}

// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
// ferrocene-annotations: fls_vnai6ag4qrdb
// Identifier Pattern Matching
