enum Foo {
    Bar { bar: bool },
    Other,
}

fn main() {
    let foo = Some(Foo::Other);

    if let Some(Foo::Bar {_}) = foo {}
    //~^ ERROR expected identifier, found reserved identifier `_`
    //~| ERROR pattern does not mention field `bar` [E0027]
}

// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
//
// ferrocene-annotations: fls_yivm43r5wnp1
// Let Statements
//
// ferrocene-annotations: fls_asj8rgccvkoe
// Struct Pattern Matching
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Underscore Pattern Matching
