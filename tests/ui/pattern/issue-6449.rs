//@ run-pass
#![allow(dead_code)]

enum Foo {
    Bar(isize),
    Baz,
}

enum Other {
    Other1(Foo),
    Other2(Foo, Foo),
}

fn main() {
    match Foo::Baz {
        ::Foo::Bar(3) => panic!(),
        ::Foo::Bar(_) if false => panic!(),
        ::Foo::Bar(..) if false => panic!(),
        ::Foo::Bar(_n) => panic!(),
        ::Foo::Baz => {}
    }
    match Foo::Bar(3) {
        ::Foo::Bar(3) => {}
        ::Foo::Bar(_) if false => panic!(),
        ::Foo::Bar(..) if false => panic!(),
        ::Foo::Bar(_n) => panic!(),
        ::Foo::Baz => panic!(),
    }
    match Foo::Bar(4) {
        ::Foo::Bar(3) => panic!(),
        ::Foo::Bar(_) if false => panic!(),
        ::Foo::Bar(..) if false => panic!(),
        ::Foo::Bar(n) => assert_eq!(n, 4),
        ::Foo::Baz => panic!(),
    }

    match Other::Other1(Foo::Baz) {
        ::Other::Other1(::Foo::Baz) => {}
        ::Other::Other1(::Foo::Bar(_)) => {}
        ::Other::Other2(::Foo::Baz, ::Foo::Bar(_)) => {}
        ::Other::Other2(::Foo::Bar(..), ::Foo::Baz) => {}
        ::Other::Other2(..) => {}
    }
}

// ferrocene-annotations: fls_d44aflefat88
// Path Pattern Matching
//
// ferrocene-annotations: fls_uloyjbaso8pz
// Path Patterns
//
// ferrocene-annotations: fls_eexupzdsu7f
// Tuple Struct Pattern Matching
//
// ferrocene-annotations: fls_vlrto778v49m
// Tuple Struct Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Underscore Pattern Matching
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Underscore Patterns
