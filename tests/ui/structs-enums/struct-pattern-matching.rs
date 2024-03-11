//@ run-pass
#![allow(non_shorthand_field_patterns)]

struct Foo {
    x: isize,
    y: isize,
}

pub fn main() {
    let a = Foo { x: 1, y: 2 };
    match a {
        Foo { x: x, y: y } => println!("yes, {}, {}", x, y)
    }

    match a {
        Foo { .. } => ()
    }
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_77scxuomlbgs
// Passing Conventions
//
// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
//
// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
//
// ferrocene-annotations: fls_jm6l7b90h6wa
// Pattern Matching
