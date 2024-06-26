//@ run-pass

#![allow(non_shorthand_field_patterns)]
#![allow(dead_code)]
#![allow(unused_variables)]

struct Foo(isize, isize, isize, isize);
struct Bar{a: isize, b: isize, c: isize, d: isize}

pub fn main() {
    let Foo(..) = Foo(5, 5, 5, 5);
    let Foo(..) = Foo(5, 5, 5, 5);
    let Bar{..} = Bar{a: 5, b: 5, c: 5, d: 5};
    let (..) = (5, 5, 5, 5);
    let Foo(a, b, ..) = Foo(5, 5, 5, 5);
    let Foo(.., d) = Foo(5, 5, 5, 5);
    let (a, b, ..) = (5, 5, 5, 5);
    let (.., c, d) = (5, 5, 5, 5);
    let Bar{b: b, ..} = Bar{a: 5, b: 5, c: 5, d: 5};
    match [5, 5, 5, 5] {
        [..] => { }
    }
    match [5, 5, 5, 5] {
        [a, ..] => { }
    }
    match [5, 5, 5, 5] {
        [.., b] => { }
    }
    match [5, 5, 5, 5] {
        [a, .., b] => { }
    }
    match [5, 5, 5] {
        [..] => { }
    }
    match [5, 5, 5] {
        [a, ..] => { }
    }
    match [5, 5, 5] {
        [.., a] => { }
    }
    match [5, 5, 5] {
        [a, .., b] => { }
    }
}

// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_asj8rgccvkoe
// Struct Pattern Matching
//
// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
//
// ferrocene-annotations: fls_eexupzdsu7f
// Tuple Struct Pattern Matching
//
// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
