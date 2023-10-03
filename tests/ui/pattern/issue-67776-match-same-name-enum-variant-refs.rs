// Test for issue #67776: binding named the same as enum variant
// should report an error even when matching against a reference type

#![allow(unused_variables)]
#![allow(non_snake_case)]

enum Foo {
    Bar,
    Baz,
}


fn fn1(e: Foo) {
    match e {
        Bar => {},
        //~^ ERROR named the same as one of the variants of the type `Foo`
        Baz => {},
        //~^ ERROR named the same as one of the variants of the type `Foo`
    }
}

fn fn2(e: &Foo) {
    match e {
        Bar => {},
        //~^ ERROR named the same as one of the variants of the type `Foo`
        Baz => {},
        //~^ ERROR named the same as one of the variants of the type `Foo`
    }
}

fn fn3(e: &mut &&mut Foo) {
    match e {
        Bar => {},
        //~^ ERROR named the same as one of the variants of the type `Foo`
        Baz => {},
        //~^ ERROR named the same as one of the variants of the type `Foo`
    }
}

fn main() {}

// ferrocene-annotations: fls_vnai6ag4qrdb
// Identifier Pattern Matching
//
// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
