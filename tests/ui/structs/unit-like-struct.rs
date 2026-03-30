//@ run-pass
struct Foo;

pub fn main() {
    let x: Foo = Foo;
    match x {
        Foo => { println!("hi"); }
    }
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
