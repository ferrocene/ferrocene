//@ build-pass (FIXME(62277): could be check-pass?)

#![deny(unused_variables)]

mod foo {
    enum Bar {}

    #[allow(dead_code)]
    pub struct Foo {
        value: Bar, // "privately" uninhabited
    }

    pub fn give_foo() -> Foo { panic!() }
}

fn main() {
    let a = 42;
    foo::give_foo();
    println!("Hello, {}", a); // ok: we can't tell that this code is dead
}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_k02nt1m5fq1z
// Panic
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
