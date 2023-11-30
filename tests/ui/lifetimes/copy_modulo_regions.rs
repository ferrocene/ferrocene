#[derive(Clone)]
struct Foo<'a>(fn(&'a ()) -> &'a ());

impl Copy for Foo<'static> {}

fn mk_foo<'a>() -> Foo<'a> {
    println!("mk_foo");
    Foo(|x| x)
}

fn foo<'a>() -> [Foo<'a>; 100] {
    [mk_foo::<'a>(); 100] //~ ERROR lifetime may not live long enough
}

fn main() {
    foo();
}

// ferrocene-annotations: fls_xinykul167l
// Array Expressions
//
// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
