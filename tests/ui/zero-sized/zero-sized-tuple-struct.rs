//@ run-pass
#![allow(unused_braces)]
#![allow(unused_assignments)]

// Make sure that the constructor args are codegened for zero-sized tuple structs

struct Foo(());

fn main() {
    let mut a = 1;
    Foo({ a = 2 });
    assert_eq!(a, 2);
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
